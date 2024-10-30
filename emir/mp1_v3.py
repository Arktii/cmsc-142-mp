import time
from bitarray import bitarray
from bitarray.util import int2ba
from items import items

# Constants
KNAPSACK_CAPACITY = 1000


def main():
    n = 2

    start = time.time()

    (solution, solution_weight, solution_value) = brgc_knapsack(items, n)

    end = time.time()

    print(f"Solution: {solution}")
    print(f"Weight: {solution_weight}")
    print(f"Value: {solution_value}")

    print(f"Total time: {end - start}")


# Uses non-recursive brgc
def brgc_knapsack(items: list[tuple[int, int]], n: int) -> tuple[bitarray, int, int]:
    solution = bitarray("0" * n)
    solution_weight = 0
    solution_value = 0

    current = bitarray("0" * n)
    current_weight = 0
    current_value = 0

    for i in range(1, 2 ** (n)):
        change_index = get_index_to_flip(i)
        current[change_index] = not current[change_index]

        print(current)

        if current[change_index] == 1:
            current_weight += items[change_index][0]
            current_value += items[change_index][1]
        else:
            current_weight -= items[change_index][0]
            current_value -= items[change_index][1]

        if current_weight <= KNAPSACK_CAPACITY and current_value > solution_value:
            solution = current.copy()
            solution_weight = current_weight
            solution_value = current_value

    return (solution, solution_weight, solution_value)


def get_index_to_flip(i: int) -> int:
    xor_value = i ^ (i - 1)
    return xor_value.bit_length() - 1


main()
