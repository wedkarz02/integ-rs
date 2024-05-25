import csv
import matplotlib.pyplot as plt

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

plt.yscale("log")
plt.xscale("log")
plt.plot(n, verr, color="red", linestyle="-", marker="", label="Vector Error")
plt.plot(n, ierr, color="green", linestyle="-", marker="", label="Integral Error")
plt.xlabel("Liczba podziałów")
plt.ylabel("Wartość błędu")
plt.title("Porównanie dokładności w wyliczaniu wartości liczby PI", loc="center", wrap=True)
plt.grid()
plt.legend()
plt.savefig("img/pi_err.png")
