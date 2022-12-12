s = 0
x = 1
i = 1
for instr in open("input.txt", "r"):
    for _ in range(2):
        if (i - 20) % 40 == 0:
            s += i * x
        if (i - 1) % 40 == 0:
            print()
        if abs((i - 1) % 40 - x) < 2:
            print("#", end="")
        else:
            print(".", end="")
        i += 1
        if "addx" not in instr:
            break
    if "addx" in instr:
        x += int(instr[5:])
print()
print(s)
