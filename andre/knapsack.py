knapsack_capacity = 1000

def knapsack(items, subsets):
    sol_value = 0
    sol_weight = 0
    solution = ''
    for set in subsets:
        weight = 0
        value = 0
        for i in range(len(set)):
            bit = int(set[i])
            if bit:
                weight += items[i][1]
                value += items[i][0]
            if weight > knapsack_capacity:
                value = 0
                break

        if value > sol_value:
            solution = set
            sol_value = value
            sol_weight = weight

    return [solution, sol_value, sol_weight]
