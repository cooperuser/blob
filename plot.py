from math import pi
import matplotlib.pyplot as plt
from matplotlib.axes import Axes
from load import get_data, Datum
from typing import List
from multiple import multiple_formatter

GLOBAL_RANGE = True

data = get_data()
mapping_dict = {
    "cyclical": 0,
    "regional": 1,
}
neurons_dict = {
    2: 0,
    3: 1,
    4: 2,
    # 5: 3,
    6: 3
}

def heatmap(axis: Axes, data: List[Datum], fitness_range: List[float], pos: List[int]):
    frequency_range = [10.0, 0.0]
    phase_range = [10.0, 0.0]
    fitnesses = []
    row = []
    last_phase = 0
    for datum in data:
        frequency = datum["frequency"]
        phase = datum["phase"]
        fitness = datum["fitness"]
        frequency_range[0] = min(frequency, frequency_range[0])
        frequency_range[1] = max(frequency, frequency_range[1])
        phase_range[0] = min(phase, phase_range[0])
        phase_range[1] = max(phase, phase_range[1])
        if phase < last_phase:
            fitnesses.append(row)
            row = []
        row.append(fitness)
        last_phase = phase

    left = 0
    right = pi
    bottom = 0
    top = 1.0
    extent = [left, right, bottom, top]
    if GLOBAL_RANGE:
        axis.imshow(fitnesses, cmap="hot", interpolation="nearest", aspect="auto", extent=extent, vmin=fitness_range[0], vmax=fitness_range[1])
    else:
        axis.imshow(fitnesses, cmap="hot", interpolation="nearest", aspect="auto", extent=extent)
    axis.set_aspect(1.0 / axis.get_data_ratio())
    axis.xaxis.set_major_locator(plt.MultipleLocator(pi / 2))
    axis.xaxis.set_minor_locator(plt.MultipleLocator(pi / 12))
    axis.xaxis.set_major_formatter(plt.FuncFormatter(multiple_formatter()))
    axis.set_ylabel([2, 3, 4, 6][pos[1]])
    axis.set_xlabel("cyclical" if not pos[0] else "regional")

fitness_range = [0.0, 0.0]
for mapping in data.keys():
    for neurons in data[mapping].keys():
        for datum in data[mapping][neurons]:
            fitness = datum["fitness"]
            fitness_range[0] = min(fitness, fitness_range[0])
            fitness_range[1] = max(fitness, fitness_range[1])
        
fig, axes = plt.subplots(nrows=4, ncols=2, sharex=True, sharey=True)
for mapping in data.keys():
    for neurons in data[mapping].keys():
        m = mapping_dict[mapping]
        n = neurons_dict[neurons]
        heatmap(axes[n, m], data[mapping][neurons], fitness_range, [m, n])

plt.show()
