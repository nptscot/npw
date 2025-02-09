import json
import os

all_stats = {}
for x in os.listdir("baseline_stats"):
    with open("baseline_stats/" + x) as f:
        all_stats[x.removesuffix(".json")] = json.load(f)

with open("baseline_stats.json", "w") as f:
    f.write(json.dumps(all_stats))
