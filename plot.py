import matplotlib.pyplot as plt
import proc

try:
    p = proc.get_proc()

    plt.figure(figsize=(7, 3))
    for i in range(2000):
        plt.clf()
        o = proc.get_output(p)
        for point in proc.get_points(o):
            plt.scatter(point[0], point[1])
        plt.xlim(-6, 1)
        plt.ylim(-1.5, 1.5)
        plt.pause(0.01)

    plt.show()
finally:
    exit()
