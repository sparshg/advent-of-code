inp = open('input.txt', 'r').read().strip().split(',')

def check1(id):
    size = len(id)
    if size % 2:
        return False
    for i in range(size // 2):
        if id[i] != id[i + size // 2]:
            return False
    return True

def dfs(i, j, id, dp):
    if i < 0:
        return len(id) % (j + 1) == 0 and j + 1 < len(id)
    if dp[i][j] != -1:
        return dp[i][j]
    dp[i][j] = False
    if id[i] == id[j]:
        dp[i][j] |= dfs(i - 1, j - 1, id, dp)
    if j == len(id) - 1:
        dp[i][j] |= dfs(i - 1, j, id, dp)
    else:
        dp[i][j] |= dfs(i, len(id) - 1, id, dp)
    return dp[i][j]

def check2(id):
    dp = [[-1] * len(id) for _ in range(len(id) - 1)]
    return dfs(len(id) - 2, len(id) - 1, id, dp)

res1, res2 = 0, 0
for [a, b] in map(lambda x: map(int, x.split('-')), inp):
    res1 += sum(filter(lambda x: check1(str(x)), range(a, b + 1)))
    res2 += sum(filter(lambda x: check2(str(x)), range(a, b + 1)))
print(res1, res2)
    