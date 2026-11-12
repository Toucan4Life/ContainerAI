"""Renders the action-abstraction audit (`container-arena --bin audit`) as one markdown table.

    python -m training.audit_report reports/audit
"""

from __future__ import annotations

import json
import sys
from pathlib import Path


def render(directory: Path) -> str:
    audits = {p.stem: json.loads(p.read_text()) for p in sorted(directory.glob("*.json"))}
    lines = [
        "# Action abstraction and information audit", "",
        "Every net plays 300 self-play games (players 3, 4, 5; seed 1) through the deployed `Neural` bot of the Rust "
        "crate `container-bots`, which offers at most 48 actions per decision (`thin`). Temperature 0 is the deployed "
        "Magnate (greedy plus repetition guard); temperature 1 samples from the net. Produced by "
        "`container-arena --bin audit` (`reports/audit/*.json`).", "",
        "| audit | games | stalled | actions/game | decisions | over the 48 cap | full-set argmax dropped | softmax mass dropped | "
        "guard overrides | `RequestLoan` while bidding or deciding (legal / chosen) |",
        "|---|---|---|---|---|---|---|---|---|---|",
    ]
    for name, d in audits.items():
        dec, th = d["decisions"], d["thinned"]
        loan = [sum(d["phases"].get(p, {}).get(k, 0) for p in ("bid", "decide", "bank_bid_cash"))
                for k in ("request_loan_legal", "request_loan_chosen")]
        guard = f"{d['guard_overrides'] / dec:.2%}" if d["config"]["temperature"] == 0 else "n/a (samples)"
        lines.append(
            f"| {name} | {d['games']} | {d['stalled']} | {d['actions_per_game']['mean']:.0f} | {dec:,} | "
            f"{d['over_cap'] / dec:.2%} | {th['full_best_dropped'] / max(th['decisions'], 1):.1%} of {th['decisions']:,} | "
            f"{th['dropped_mass']['mean']:.1%} | {guard} | {loan[0]:,} / {loan[1]} |")
    lines += ["", "Over the cap, the thinning keeps `i*(n-1)/47` of the n actions, so the endpoints (`Bid(0)`, `Bid(cash)`, "
              "`DoneBidding`, `RequestLoan`) always survive. \"Full-set argmax dropped\" and \"softmax mass dropped\" "
              "evaluate the net on the unthinned list at decisions where thinning applied; the nets were trained on the "
              "thinned list, so these describe how smooth their bid preferences are, not a measured loss.", ""]
    deployed = audits.get("magnate_deployed")
    if deployed:
        lines += ["## Deployed Magnate by phase", "",
                  "| phase | decisions | legal actions: mean | p99 | max | over the cap | cash p50 | cash p99 | `RequestLoan` legal / chosen |",
                  "|---|---|---|---|---|---|---|---|---|"]
        for phase, p in deployed["phases"].items():
            cash = p["cash"] or {}
            lines.append(
                f"| {phase} | {p['decisions']:,} | {p['legal']['mean']:.1f} | {p['legal']['p99']} | {p['legal']['max']} | "
                f"{p['over_cap']:,} | {cash.get('p50', '')} | {cash.get('p99', '')} | "
                f"{p['request_loan_legal']:,} / {p['request_loan_chosen']:,} |")
        lines.append("")
    return "\n".join(lines)


if __name__ == "__main__":
    directory = Path(sys.argv[1] if len(sys.argv) > 1 else "reports/audit")
    text = render(directory)
    (directory / "summary.md").write_text(text)
    print(text)
