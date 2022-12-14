import subprocess
import csv

BIN = "./target/x86_64-pc-windows-msvc/release/blob.exe"
FILE = "./data.csv"

data = []
max_params = {"fitness": 0}
for frequency in range(10, 210):
    for phase in range(2, 13):
        x = subprocess.check_output([BIN, str(frequency), str(phase), "--nogui"])
        fitness = float(x)
        datum = {"frequency": frequency, "phase": phase, "fitness": fitness}
        data.append(datum)
        if datum["fitness"] > max_params["fitness"]:
            max_params = datum
        print(frequency, phase, fitness)

print(max_params)

with open(FILE, "w", newline="") as csvfile:
    writer = csv.DictWriter(csvfile, fieldnames=["frequency", "phase", "fitness"])
    writer.writeheader()
    writer.writerows(data)
