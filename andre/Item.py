import random

max_weight = 100
min_weight = 50

max_value = 500
min_value = 100

def generate_items(n):
    items = []
    for _ in range(n):
        value = random.randint(min_value, max_value)
        weight = random.randint(min_weight, max_weight)
        item = (value, weight)
        items.append(item)

    return items