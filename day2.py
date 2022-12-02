wins = {'C': 'X', 'A': 'Y', 'B': 'Z'}
draws = {'A': 'X', 'B': 'Y', 'C': 'Z'}
loses = {'B': 'X', 'C': 'Y', 'A': 'Z'}
s = 0
for line in open('input.txt', 'r').readlines():
    k, v = line.strip().split(' ')
    if (k, v) in wins.items():
        s += 6
    elif (k, v) in draws.items():
        s += 3
    s += ord(v) - 87
print(s)

s = 0
for line in open('input.txt', 'r').readlines():
    v, k = line.strip().split(' ')
    if k == 'X':
        s += ord(loses[v]) - 87
    elif k == 'Y':
        s += 3 + ord(draws[v]) - 87
    elif k == 'Z':
        s += 6 + ord(wins[v]) - 87
print(s)
