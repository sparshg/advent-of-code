import re
s = 0
t = 0
for line in open('input.txt', 'r'):
    p1, p2, p3, p4 = map(int, re.split(",|-", line))
    if p1 >= p3 and p2 <= p4 or p1 <= p3 and p2 >= p4:
        s += 1
    if p2 >= p3 and p1 <= p4:
        t += 1
print(s, t)