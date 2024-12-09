from bitarray import bitarray
import time

def brgc(n):
    if n == 1:
        return [bitarray('0'),bitarray('1')]
    else:
        l1 = brgc(n-1)
        l2 = l1[::-1].copy()

        for i in range(len(l1)):
            l1[i] =  bitarray('0') + l1[i]
            l2[i] =  bitarray('1') + l2[i]

        return l1 + l2

# https://stackoverflow.com/questions/22039562/non-recursive-grey-code-algorithm-understanding
def brgc_non_recursive(n):
    l = [bitarray('0' * n)]
    count = 2 ** n

    for i in range(1, count):
        g = l[-1].copy()
        index = lsb(i)
        g[n - index - 1] = not g[n - index - 1]
        l.append(g)

    return l

def lsb(i):
    pos = 0
    while (i & 1) == 0:
        i >>= 1 #bit shift
        pos += 1
    return pos

# print(brgc(3))
# print(brgc_non_recursive(3))
# start_time = time.time()
# d = brgc_non_recursive(25)
# end_time = time.time()
# print(end_time - start_time)

# start_time = time.time()
# d = brgc(25)
# end_time = time.time()
# print(end_time - start_time)
