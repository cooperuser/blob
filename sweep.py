import subprocess
import csv
from math import floor, pi

BIN = "./target/x86_64-pc-windows-msvc/release/blob.exe"

def sweep(path, neurons):
    data = []
    frequency = 0.01
    index = 0
    while frequency <= 1.0:
        phase = pi / 12
        while phase <= pi:
            x = subprocess.check_output([BIN, str(frequency), str(phase), str(neurons), "--nogui"])
            fitness = float(x)
            datum = {"frequency": frequency, "phase": phase, "fitness": fitness}
            data.append(datum)
            percent = index / (100 * 12)
            print(floor(percent * 10000) / 100, ":", frequency, phase, fitness)
            phase += pi / 12
            index += 1
        frequency += 0.01

    with open(path, "w", newline="") as csvfile:
        writer = csv.DictWriter(csvfile, fieldnames=["frequency", "phase", "fitness"])
        writer.writeheader()
        writer.writerows(data)


if __name__ == "__main__":
    for n in range(3, 6):
        sweep("./data/mapping-regional_neurons-" + str(n) + ".csv", n)
