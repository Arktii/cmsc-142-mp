import random

# Constants
FILE_NAME = "items.rs"

ITEM_COUNT = 100_000
WEIGHT_RANGE = range(100, 1501)
VALUES_RANGE = range(100, 501)

# Write to file
f = open(FILE_NAME, "w")

f.write("// Pregenerated using Python code\n")

f.write("use crate::item::Item;\n")

f.write("pub const ITEM_SETS: [[Item; " + str(ITEM_COUNT) + "]; 3] = [\n")
for i in range(3):
    # Generate List
    items = []
    for _ in range(ITEM_COUNT):
        items.append((random.choice(WEIGHT_RANGE), random.choice(VALUES_RANGE)))

    f.write("\t[\n")
    for item in items:
        f.write("\t\tItem {\n")
        f.write("\t\t\tweight: " + str(item[0]) + ", \n")
        f.write("\t\t\tvalue: " + str(item[1]) + ", \n")
        f.write("\t\t},\n")
    f.write("\t],\n")
f.write("];\n")

f.close()
