from os import listdir
from os.path import isfile, join
import csv
import re
from typing import Dict, List

Datum = Dict[str, float]

def get_data() -> Dict[str, Dict[int, List[Datum]]]:
    files = [f for f in listdir("data") if isfile(join("data", f))]
    data = {}

    for file in files:
        path = join("data", file)
        match = re.match("mapping-([a-z]+)_.*-([\\d\\.]+)\\.csv", file)
        if not match:
            exit()
        mapping = match.groups()[0]
        neurons = int(match.groups()[1])
        if mapping not in data:
            data[mapping] = {}
        data[mapping][neurons] = []

        with open(path, 'r', newline="") as csvfile:
            reader = csv.DictReader(csvfile, ["frequency", "phase", "fitness"])
            for i, row in enumerate(reader):
                if i == 0: continue
                frequency = float(row["frequency"])
                phase = float(row["phase"])
                fitness = float(row["fitness"])
                d = {
                    "frequency": frequency,
                    "phase": phase,
                    "fitness": fitness
                }
                data[mapping][neurons].append(d)
    return data
