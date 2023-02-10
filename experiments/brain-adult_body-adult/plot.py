import matplotlib.pyplot as plt
from load import get_data

data = get_data()

for run in data:
    plt.plot([d["position"] for d in run])

plt.xticks([i * 6000 for i in range(7)], [i * 100 for i in range(7)])
plt.xlabel("time")
plt.ylabel("x position")
plt.show()
