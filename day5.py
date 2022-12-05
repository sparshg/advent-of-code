# part = 1
part = 2

import re
n = 9
stacks = ["" for _ in range(n)]
stacking = True
for line in open('input.txt', 'r'):
    if line == '\n': continue
    if stacking:
        for i in range(n):
            char = line[1 + i * 4]
            if char == '1':
                stacking = False
                break
            if char != ' ':
                stacks[i] += line[1 + i * 4]
    else:
        a, b, c = map(int, re.findall(r'\d+', line))
        if part == 1:
            stacks[c - 1] = stacks[b - 1][:a][::-1] + stacks[c - 1]
        else:
            stacks[c - 1] = stacks[b - 1][:a] + stacks[c - 1]
        stacks[b - 1] = stacks[b - 1][a:]
print(''.join([i[0] for i in stacks]))