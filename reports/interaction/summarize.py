"""Prints the baseline JSON files as one table: `python reports/interaction/summarize.py [DIR] [PREFIX]`."""
import glob, json, os, sys

d = sys.argv[1] if len(sys.argv) > 1 else os.path.dirname(__file__)
prefix = sys.argv[2] if len(sys.argv) > 2 else "baseline_"
cols = [("win_share", "win"), ("win_ci95", "±"), ("score", "score"), ("cash", "cash"), ("island", "island"), ("leftovers", "left"), ("loans", "loans"), ("bought", "buy"), ("mean_price_paid", "$paid"),
        ("sold", "sold"), ("bids", "bids"), ("mean_bid", "$bid"), ("auctions_won", "won"),
        ("bank_actions", "bank"), ("listed_fair_share", "list≤3"), ("cheap_take_rate", "cheapTake"),
        ("stall_rate", "stall")]
print(f"{'file':34}{'seat':12}" + "".join(f"{c[1]:>9}" for c in cols))
for f in sorted(glob.glob(os.path.join(d, prefix + "*.json"))):
    data = json.load(open(f))
    name = os.path.basename(f)[len(prefix):-5]
    focus = data["bots"].get("focus")
    if focus:
        print(f"{name:34}{'focus':12}" + "".join(f"{focus.get(c[0], float("nan")):9.3f}" for c in cols))
