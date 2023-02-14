import matplotlib.pyplot as plt
from load import get_data

data = get_data()
fig, axes = plt.subplots(nrows=2, ncols=1, sharex=True)

for run in data["cyclical"]:
    axes[0].plot([d["position"] for d in run])

for run in data["regional"]:
    axes[1].plot([d["position"] for d in run])

axes[0].set_ylabel("cyclical")
axes[1].set_ylabel("regional")

plt.xticks([i * 3600 for i in range(11)], [i for i in range(11)])
plt.xlabel("time (minutes)")
# plt.ylabel("displacement")
plt.show()
