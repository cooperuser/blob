import matplotlib.pyplot as plt
import csv

FILE = "./data.csv"

data = {}
phase_range = [10, 0]
frequency_range = [100, 0]
fitness_range = [0.0, 0.0]
with open(FILE, "r", newline="") as csvfile:
    reader = csv.DictReader(csvfile, fieldnames=["frequency", "phase", "fitness"])
    for i, row in enumerate(reader):
        if i == 0: continue
        phase = int(row["phase"])
        frequency = int(row["frequency"])
        fitness = float(row["fitness"])
        phase_range[0] = min(phase_range[0], phase)
        phase_range[1] = max(phase_range[1], phase)
        frequency_range[0] = min(frequency_range[0], frequency)
        frequency_range[1] = max(frequency_range[1], frequency)
        fitness_range[0] = min(fitness_range[0], fitness)
        fitness_range[1] = max(fitness_range[1], fitness)
        data[(phase, frequency)] = fitness

fitnesses = []
for phase in range(phase_range[0], phase_range[1] + 1):
    row = []
    for frequency in range(frequency_range[0], frequency_range[1] + 1):
        fitness = data[(phase, frequency)]
        row.append(fitness)
    for _ in range(18):
        fitnesses.append(row)

plt.imshow(fitnesses, cmap="hot", interpolation="nearest")
plt.ylabel("phase")
plt.xlabel("frequency*")

plt.show()
