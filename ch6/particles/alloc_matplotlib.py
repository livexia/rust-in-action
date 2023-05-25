import matplotlib.pyplot as plt

with open("alloc.tsv") as f:
    read_data = f.readlines()

data = [line.split("\t", 1) for line in read_data]

sizes = [line[0] for line in data]
times = [line[1] for line in data]

print(f"data count: {len(data)}, size count: {len(sizes)}, time count: {len(times)}")

plt.plot(times, sizes, "r.")
plt.xscale("log", base=2)
plt.yscale("log")
plt.xlabel("Allocation size (bytes)")
plt.ylabel("Allocation duration (ns)")
plt.axis([0, 10000, 10, 1000])
plt.show()
