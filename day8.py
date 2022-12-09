grid = [list(map(int, list(x))) for x in open('input.txt', 'r').read().splitlines()]
ans = [[0] * len(grid[0]) for _ in range(len(grid))]

top = [-1] * len(grid[0])
bottom = [-1] * len(grid[0])

for j, rows in enumerate(zip(grid, reversed(grid))):
    # Row from top, bottom
    tr, br = rows
    left, right = -1, -1
    for i, trees in enumerate(zip(tr, reversed(tr))):
        # Tree from left, right
        lt, rt = trees
        if lt > left:
            ans[j][i] = 1
            left = lt
        if rt > right:
            ans[j][len(ans[j]) - i - 1] = 1
            right = rt
        if tr[i] > top[i]:
            ans[j][i] = 1
            top[i] = tr[i]
        if br[i] > bottom[i]:
            ans[len(ans) - j - 1][i] = 1
            bottom[i] = br[i]
print(sum([x.count(1) for x in ans]))

score = 0
for j, row in enumerate(grid):
    for i, tree in enumerate(row):
        a, b, c, d = 0, 0, 0, 0
        for k in row[i + 1:]:
            a += 1
            if k >= tree:
                break
        for k in reversed(row[:i]):
            b += 1
            if k >= tree:
                break
        for k in range(j + 1, len(grid)):
            c += 1
            if grid[k][i] >= tree:
                break
        for k in reversed(range(j)):
            d += 1
            if grid[k][i] >= tree:
                break
        score = max(score, a*b*c*d)
print(score)
