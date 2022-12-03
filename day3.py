s = 0
for line in open('input.txt', 'r'):
    first, second = line[:len(line)//2], line[len(line)//2:]
    s += [ord(c) - 96 if ord(c) > 96 else ord(c) - 38 for c in first if c in second][0]
print(s)

s = 0
for i, line in enumerate(open('input.txt', 'r')):
    if i % 3 == 0:
        first = line
    elif i % 3 == 1:
        second = line
    else:
        third = line
        s += [ord(c) - 96 if ord(c) > 96 else ord(c) - 38 for c in first if c in second and c in third][0]
print(s)