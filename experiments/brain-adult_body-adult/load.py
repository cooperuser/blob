from os import listdir
from os.path import isfile, join
import csv
from typing import Dict, List

Datum = Dict[str, float]

def get_data() -> List[List[Datum]]:
    files = [f for f in listdir("data") if isfile(join("data", f))]
    data = []

    for file in files:
        path = join("data", file)
        with open(path, "r", newline="") as csvfile:
            reader = csv.DictReader(csvfile, ["time", "position"])
            run = []
            for row in reader:
                run.append({
                    "time": float(row["time"]),
                    "position": float(row["position"])
                })
            data.append(run)

    return data
