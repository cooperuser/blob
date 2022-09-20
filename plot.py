import matplotlib.pyplot as plt
import proc

p = proc.get_proc()

for i in range(100):
    plt.clf()
    o = proc.get_output(p)
    for point in proc.get_points(o):
        plt.scatter(point[0], point[1])
    plt.pause(0.01)

plt.show()
