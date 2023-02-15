import subprocess
import csv
from math import pi
from tqdm.contrib import itertools

BIN = "./target/x86_64-pc-windows-msvc/release/blob.exe"

def sweep(path, segments):
    data = []
    for frequency, phase in itertools.product([(f + 1) / 100 for f in range(100)], [(p + 1) * pi / 12 for p in range(12)]):
        x = subprocess.check_output([BIN, str(frequency), str(phase), "4", str(segments), "--nogui"])
        fitness = float(x)
        datum = {"frequency": frequency, "phase": phase, "fitness": fitness}
        data.append(datum)

    with open(path, "w", newline="") as csvfile:
        writer = csv.DictWriter(csvfile, fieldnames=["frequency", "phase", "fitness"])
        writer.writeheader()
        writer.writerows(data)


if __name__ == "__main__":
    for s in range(6, 8):
        sweep("./data/mapping-regional_segments-" + str(s) + ".csv", s)
