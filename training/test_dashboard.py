import json
import threading
import urllib.error
import urllib.request
from http.server import ThreadingHTTPServer

from training.dashboard.server import make_handler, thin


def serve(root):
    server = ThreadingHTTPServer(("127.0.0.1", 0), make_handler(root))
    threading.Thread(target=server.serve_forever, daemon=True).start()
    return server, f"http://127.0.0.1:{server.server_port}"


def get(url):
    with urllib.request.urlopen(url) as r:
        return r.read()


def test_serves_runs_and_survives_a_half_written_line(tmp_path):
    run = tmp_path / "a"
    run.mkdir()
    rows = [{"type": "train", "t": 1, "games_per_sec": 50}, {"type": "eval", "t": 2, "elo": 900.5}]
    (run / "metrics.jsonl").write_text("\n".join(map(json.dumps, rows)) + '\n{"type": "tr')
    (run / "status.json").write_text(json.dumps({"state": "running", "elapsed": 2, "budget": 10}))
    server, base = serve(tmp_path)
    try:
        listing = json.loads(get(base + "/api/runs"))
        assert listing[0]["name"] == "a" and listing[0]["best_elo"] == 900.5
        assert len(json.loads(get(base + "/api/run/a"))["metrics"]) == 2
        assert b"Container" in get(base + "/")
        assert isinstance(json.loads(get(base + "/api/ladder")), dict)
        for bad in ("/api/run/..", "/api/run/nope"):
            try:
                get(base + bad)
                raise AssertionError("should be 404")
            except urllib.error.HTTPError as e:
                assert e.code == 404
    finally:
        server.shutdown()


def test_thin_keeps_evals_and_bounds_training_rows():
    rows = [{"type": "train", "t": i} for i in range(100)] + [{"type": "eval", "t": 50.5, "elo": 1}]
    out = thin(rows, limit=10)
    assert sum(r["type"] == "train" for r in out) == 10 and any(r["type"] == "eval" for r in out)
