"""Serves the training dashboard and the data it charts.

    python -m training.dashboard.server [--runs runs] [--port 8765]

Reads `runs/<name>/{metrics.jsonl,status.json,config.json}` and `training/ladder.json`; it never
writes. The page polls `/api/runs`, so it follows a run that is still training.
"""

from __future__ import annotations

import argparse
import json
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import unquote, urlparse

HERE = Path(__file__).resolve().parent
LADDER = HERE.parent / "ladder.json"
MAX_POINTS = 4000


def read_json(path: Path):
    try:
        return json.loads(path.read_text())
    except (OSError, ValueError):
        return None


def read_metrics(path: Path) -> list[dict]:
    rows = []
    try:
        with open(path) as f:
            for line in f:
                try:
                    rows.append(json.loads(line))
                except ValueError:
                    pass  # the line being written right now
    except OSError:
        pass
    return rows


def thin(rows: list[dict], limit: int = MAX_POINTS) -> list[dict]:
    """Keeps every evaluation and spreads the rest evenly, so long runs stay light to chart."""
    train = [r for r in rows if r.get("type") == "train"]
    other = [r for r in rows if r.get("type") != "train"]
    if len(train) > limit:
        step = len(train) / limit
        train = [train[int(i * step)] for i in range(limit)]
    return sorted(train + other, key=lambda r: r.get("t", 0))


def run_names(root: Path) -> list[str]:
    if not root.is_dir():
        return []
    return sorted((p.name for p in root.iterdir() if (p / "metrics.jsonl").exists()),
                  key=lambda n: (root / n / "metrics.jsonl").stat().st_mtime, reverse=True)


def summary(root: Path, name: str) -> dict:
    rows = read_metrics(root / name / "metrics.jsonl")
    evals = [r for r in rows if r.get("type") == "eval"]
    return {"name": name, "status": read_json(root / name / "status.json"),
            "config": read_json(root / name / "config.json"),
            "best_elo": max((e["elo"] for e in evals), default=None),
            "last_elo": evals[-1]["elo"] if evals else None}


def make_handler(root: Path):
    class Handler(BaseHTTPRequestHandler):
        def send(self, code: int, body: bytes, kind: str) -> None:
            self.send_response(code)
            self.send_header("Content-Type", kind)
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(body)

        def json(self, value, code: int = 200) -> None:
            self.send(code, json.dumps(value).encode(), "application/json")

        def do_GET(self) -> None:
            path = unquote(urlparse(self.path).path)
            if path in ("/", "/index.html"):
                self.send(200, (HERE / "index.html").read_bytes(), "text/html; charset=utf-8")
            elif path == "/api/runs":
                self.json([summary(root, n) for n in run_names(root)])
            elif path == "/api/ladder":
                self.json(read_json(LADDER) or {})
            elif path.startswith("/api/run/"):
                name = path.removeprefix("/api/run/")
                if name not in run_names(root):  # also rules out path tricks
                    return self.json({"error": "no such run"}, 404)
                self.json({**summary(root, name), "metrics": thin(read_metrics(root / name / "metrics.jsonl"))})
            else:
                self.send(404, b"not found", "text/plain")

        def log_message(self, *args) -> None:
            pass

    return Handler


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--runs", default="runs")
    ap.add_argument("--port", type=int, default=8765)
    ap.add_argument("--host", default="127.0.0.1")
    args = ap.parse_args()
    server = ThreadingHTTPServer((args.host, args.port), make_handler(Path(args.runs)))
    print(f"dashboard on http://{args.host}:{args.port}  (runs: {args.runs})")
    server.serve_forever()


if __name__ == "__main__":
    main()
