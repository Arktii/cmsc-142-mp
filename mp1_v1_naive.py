import time
from bitarray import bitarray
from items import items

# Constants
KNAPSACK_CAPACITY = 1000


def main():
    total_start = time.time()

    gen_start = time.time()
    all_subsets = binary_reflected_gray_code(20)
    gen_end = time.time()

    search_start = time.time()
    (solution, solution_weight, solution_value) = knapsack(items, all_subsets)
    search_end = time.time()

    total_end = time.time()

    print(f"Solution: {solution}")
    print(f"Weight: {solution_weight}")
    print(f"Value: {solution_value}")

    print(f"Total time: {total_end - total_start}")
    print(f"Generation Time: {gen_end - gen_start}")
    print(f"Search Time: {search_end - search_start}")


def binary_reflected_gray_code(n: int) -> list[bitarray]:
    if n == 1:
        return [bitarray("0"), bitarray("1")]
    else:
        list1 = binary_reflected_gray_code(n - 1)
        list2 = [x.copy() for x in list1]  # deep copy
        list2.reverse()

        for element in list1:
            element.append(0)

        for element in list2:
            element.append(1)

        return list1 + list2


# Starting with naive approach
def knapsack(
    items: list[tuple[int, int]], all_subsets: list[bitarray]
) -> tuple[bitarray, int, int]:
    solution = all_subsets[0]
    solution_weight = 0
    solution_value = 0

    for candidate in all_subsets:
        weight = calculate_weight(items, candidate)
        if weight > KNAPSACK_CAPACITY:
            continue

        value = calculate_value(items, candidate)
        if value >= solution_value:
            solution = candidate
            solution_weight = weight
            solution_value = value

    return (solution, solution_weight, solution_value)


def calculate_weight(items: list[tuple[int, int]], item_set: bitarray) -> int:
    weight = 0
    for i in range(len(item_set)):
        if item_set[i] == 1:
            weight += items[i][0]
    return weight


def calculate_value(items: list[tuple[int, int]], item_set: bitarray) -> int:
    value = 0
    for i in range(len(item_set)):
        if item_set[i] == 1:
            value += items[i][1]
    return value


main()
