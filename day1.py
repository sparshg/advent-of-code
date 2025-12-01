inp = open('input.txt', 'r').read().strip().split('\n')
ptr, res1, res2 = 50, 0, 0

for op, num in map(lambda x: (x[0], int(x[1:])), inp):
    ptr = 100 if ptr == 0 and op == 'L' else ptr
    ptr = (ptr - num) if op == 'L' else (ptr + num)
    res2 += -((ptr - 1) // 100) if op == 'L' else ptr // 100
    ptr %= 100
    res1 += ptr == 0
print(res1, res2)