import time
from Item import generate_items
from brgc import brgc
from knapsack import knapsack

item_count = 15
subsets = brgc(item_count)

def main():
    trials = 3
    f = open("output.txt", "w")
    g = open("output.txt", "a")
    for trial in range(1, trials + 1):
        items = generate_items(item_count)
        print(f"\nTrial {trial}:")

        start_time = time.time()
        result = knapsack(items, subsets)
        end_time = time.time()

        print(f"Time taken: {end_time - start_time}")
        print(f"Knapsack result: {result}")

        f.write(f"Trial {trial}:\n")
        f.write(f"\tKnapsack result: {result}\n")
        f.write(f"\tTime taken: {end_time - start_time}\n\n")

        g.write(f"Trial {trial}:\n\t{items}\n\n")

if __name__ == "__main__":
    main()
