from os import listdir
from os.path import isfile, join
import csv
from typing import Dict, List

Datum = Dict[str, float]

def get_data() -> Dict[str, List[List[Datum]]]:
    files = [f for f in listdir("data") if isfile(join("data", f))]
    data = {
        "cyclical": [],
        "regional": []
    }

    for file in files:
        path = join("data", file)
        type = "regional" if file[0] == 'r' else "cyclical"
        with open(path, "r", newline="") as csvfile:
            reader = csv.DictReader(csvfile, ["time", "position"])
            run = []
            for row in reader:
                run.append({
                    "time": float(row["time"]),
                    "position": float(row["position"])
                })
            data[type].append(run)

    return data
