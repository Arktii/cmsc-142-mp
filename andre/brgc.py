# TODO: use bitarray

def brgc(n):
    if n == 1:
        return ['0','1']
    else:
        l1 = brgc(n-1)
        l2 = l1[::-1]

        for i in range(len(l1)):
            l1[i] = '0' + l1[i]
            l2[i] = '1' + l2[i]

        return l1 + l2
