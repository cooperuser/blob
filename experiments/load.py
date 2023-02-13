from os import listdir
from os.path import isfile, join
import csv
from typing import Dict, List, Tuple

Datum = Dict[str, float]

def get_data_single(dir) -> Dict[str, List[List[Datum]]]:
    data_dir = join(dir, "data")
    files = [f for f in listdir(data_dir) if isfile(join(data_dir, f))]
    data = {
        "cyclical": [],
        "regional": []
    }

    for file in files:
        path = join(data_dir, file)
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

def get_data() -> Dict[str, Dict[Tuple[bool, bool], List[List[Datum]]]]:
    orig = {
        (False, False): get_data_single("brain-adult_body-adult"),
        (False, True): get_data_single("brain-adult_body-growing"),
        (True, False): get_data_single("brain-growing_body-adult"),
        (True, True): get_data_single("brain-growing_body-growing"),
    }
    data = { "cyclical": {}, "regional": {} }
    data = {mapping: {key: orig[key][mapping] for key in orig} for mapping in data}
    return data
