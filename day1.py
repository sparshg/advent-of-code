inp = open('input.txt', 'r').read().strip().split('\n\n')
inp = list(map(lambda i: sum([int(x) for x in i.split('\n')]), inp))
print(max(inp))
print(sum(sorted(inp, reverse=True)[:3]))