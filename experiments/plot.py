import matplotlib.pyplot as plt
from matplotlib.axes import Axes
import statistics
from load import get_data, Datum
from typing import List

data = get_data()
fig, axes = plt.subplots(nrows=2, ncols=1, sharex=True)
labels = {
    (False, False): "brain: adult, body: adult",
    (False, True): "brain: adult, body: growing",
    (True, False): "brain: growing, body: adult",
    (True, True): "brain: growing, body: growing",
}

def aggregate(axes: Axes, data: List[List[Datum]], label: str):
    if len(data) == 0:
        return

    means = []
    lower = []
    upper = []

    for run in data:
        if len(run) == 36008:
            run.pop()

    for time in range(len(data[0])):
        try:
            d = [run[time]["position"] for run in data]
            mean = statistics.mean(d)
            sd = statistics.stdev(d)
            means.append(mean)
            lower.append(mean - sd)
            upper.append(mean + sd)
        except IndexError:
            pass

    axes.plot(range(len(data[0])), means, label=label)
    axes.fill_between(range(len(data[0])), lower, upper, alpha=0.25)

for exp in data["cyclical"]:
    aggregate(axes[0], data["cyclical"][exp], labels[exp])

for exp in data["regional"]:
    aggregate(axes[1], data["regional"][exp], labels[exp])

axes[0].set_ylabel("cyclical")
axes[1].set_ylabel("regional")

axes[0].legend()

plt.xticks([i * 600 for i in range(11)], [i for i in range(11)])
plt.xlabel("time (minutes)")
# plt.ylabel("displacement")
plt.show()
