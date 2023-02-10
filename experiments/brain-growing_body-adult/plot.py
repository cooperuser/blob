import matplotlib.pyplot as plt
from load import get_data

data = get_data()

for run in data["regional"]:
    plt.plot([d["position"] for d in run])

plt.xticks([i * 3600 for i in range(11)], [i + 2 for i in range(11)])
plt.xlabel("# of neurons")
plt.ylabel("displacement")
plt.show()
