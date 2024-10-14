import random

# Constants
FILE_NAME = "items3.py"

ITEM_COUNT = 50
WEIGHT_RANGE = range(50, 101)
VALUES_RANGE = range(100, 501)

f = open(FILE_NAME, "w")

# Generate list
items = []
for _ in range(ITEM_COUNT):
    items.append((random.choice(WEIGHT_RANGE), random.choice(VALUES_RANGE)))

# Write to file
f.write("items = [\n")
for item in items:
    f.write("\t(" + str(item[0]) + ", " + str(item[1]) + "),\n")
f.write("]")

f.close()