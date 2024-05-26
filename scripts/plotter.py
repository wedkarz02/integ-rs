import csv
import matplotlib.pyplot as plt
import sys
import numpy as np


def plot(n, data, colors, labels, xl, yl, title, out_path):
    plt.figure()
    plt.yscale("log")
    plt.xscale("log")
    for i in range(len(data)):
        plt.plot(n, data[i], color=colors[i],
                 linestyle="-", marker="", label=labels[i])

    plt.xlabel(xl)
    plt.ylabel(yl)
    plt.title(title, loc="center", wrap=True)
    plt.grid()
    plt.legend()
    plt.savefig(out_path)


def load_data_cmp(path):
    n = []
    rec = []
    trp = []
    sim = []
    with open(path, "r") as f:
        reader = csv.reader(f, delimiter=";")
        next(reader)
        for row in reader:
            n.append(int(row[0]))
            rec.append(float(row[1]))
            trp.append(float(row[2]))
            sim.append(float(row[3]))

    return n, [rec, trp, sim]


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Invalid number of arguments", file=sys.stderr)
        sys.exit(1)

    option = sys.argv[1]

    if option == "pi":
        n = []
        verr = []
        ierr = []
        with open("dump/pi_res.csv", "r") as f:
            reader = csv.reader(f, delimiter=";")
            next(reader)
            for row in reader:
                n.append(int(row[0]))
                verr.append(float(row[2]))
                ierr.append(float(row[4]))

        data = [verr, ierr]
        plot(n, data, ["red", "green"], ["Vector Error", "Integral Error"], "Liczba podziałów",
             "Wartość błędu", "Porównanie dokładności w wyliczaniu wartości liczby PI", "img/pi_err.png")
    elif option == "all":
        files = ["1x", "cos", "ex", "sin", "x2"]
        for name in files:
            n, data = load_data_cmp(f"dump/{name}.csv")
            plot(n, data, ["red", "green", "blue"], ["Rectangle Error", "Trapezoid Error", "Simpson's Error"], "Liczba podziałów",
                 "Wartość błędu", "Porównanie dokładności metod całkowania", f"img/{name}_err.png")
    else:
        print("Unknown argument", file=sys.stderr)
