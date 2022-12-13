from functools import cmp_to_key


def parse(s):
    ans = []
    if s == "[]":
        return ans
    i, j, brackets = 1, 1, 0
    while j < len(s):
        if brackets == 0:
            if s[j] in (",", "]"):
                ans.append(int(s[i:j]))
                i = j + 1
        if s[j] == "[":
            brackets += 1
        elif s[j] == "]":
            brackets -= 1
            if brackets == 0:
                j += 1
                ans.append(parse(s[i:j]))
                i = j + 1
        j += 1
    return ans


def compare(a, b):
    if type(a) is int and type(b) is int:
        return 1 if a - b < 0 else -1 if a - b > 0 else 0
    if type(a) is int:
        a = [a]
    if type(b) is int:
        b = [b]
    for i, j in zip(a, b):
        c = compare(i, j)
        if c != 0:
            return c
    if len(a) < len(b):
        return 1
    if len(a) > len(b):
        return -1
    return 0


s = 0
lst = []

for i, pair in enumerate(open("input.txt", "r").read().split("\n\n")):
    a, b = map(parse, pair.splitlines())
    lst += [a, b]
    if compare(a, b) == 1:
        s += i + 1

lst += [[[2]], [[6]]]
lst.sort(key=cmp_to_key(compare), reverse=True)
print(s)

s = 1
for i, x in enumerate(lst):
    if x in ([[2]], [[6]]):
        s *= i + 1
print(s)
