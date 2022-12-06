inp = open('input.txt', 'r').read().strip()

for i in range(len(inp)):
    if len(set(inp[i:i+4])) == 4:
        print(i + 4)
        break

for i in range(len(inp)):
    if len(set(inp[i:i+14])) == 14:
        print(i + 14)
        break