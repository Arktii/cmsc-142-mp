import time
from bitarray import bitarray
from items import items

# Constants
KNAPSACK_CAPACITY = 1000

def main():
    total_start = time.time()

    gen_start = time.time()
    all_subsets = binary_reflected_gray_code(26)
    gen_end = time.time()



    print(all_subsets[1:10])

    print(f"Time: {gen_end - gen_start}")


def binary_reflected_gray_code(n: int) -> list[bitarray]:
    if n == 1:
        return [bitarray("0"), bitarray("1")]
    else:
        list1 = binary_reflected_gray_code(n - 1)
        list2 = [x.copy() for x in list1] # deep copy
        list2.reverse()

        for element in list1:
            element.insert(0, 0)

        for element in list2:
            element.insert(0, 1)
            
        return list1 + list2


main()
    