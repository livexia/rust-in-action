import matplotlib.pyplot as plt

with open("alloc.tsv") as f:
    read_data = f.readlines()

data = [line.split("\t", 1) for line in read_data]

print(f"data count: {len(data)}")

sizes = [int(line[0]) for line in data]
times = [int(line[1]) for line in data]

print(f"size count: {len(sizes)}, time count: {len(times)}")
print(f"min size: {min(sizes)}, max size: {max(sizes)}")
print(f"min time: {min(times)}, max time: {max(times)}")

plt.plot(sizes, times, "r.")
plt.xscale("log", base=2)
plt.yscale("log")
plt.xlabel("Allocation size (bytes)")
plt.ylabel("Allocation duration (ns)")
plt.axis([1, 100000, 10, 10000])
plt.minorticks_on()
plt.show()
