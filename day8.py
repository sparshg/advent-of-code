from itertools import combinations
from typing import Self

def dist_sq(p1, p2):
    return (p1[0] - p2[0]) * (p1[0] - p2[0]) + (p1[1] - p2[1]) * (p1[1] - p2[1]) + (p1[2] - p2[2]) * (p1[2] - p2[2])

inp = [tuple(map(int, pos.split(','))) for pos in open('input.txt', 'r').read().splitlines()]
dists = sorted([(p1, p2) for p1, p2 in combinations(inp, 2)], key = lambda p: dist_sq(p[0], p[1]))

class Circuit:
    circuits: list[Self] = []

    def __init__(self, pos):
        self.pos = set([pos])
        self.circuits.append(self)

    @staticmethod
    def of(pos) -> Self:
        for c in Circuit.circuits:
            if pos in c.pos:
                return c
        return Circuit(pos)
    
    def merge(self, c: Self):
        if c == self:
            return False
        self.pos |= c.pos
        self.circuits.remove(c)
        return True

part1, part2 = dists[:1000], dists[1000:]

for p1, p2 in part1:
    p1, p2 = map(Circuit.of, (p1, p2))
    p1.merge(p2)

c1, c2, c3 = sorted(map(lambda x: len(x.pos), Circuit.circuits))[-3:]
print(c1 * c2 * c3)

for p1, p2 in part2:
    x1, x2 = map(Circuit.of, (p1, p2))
    if x1.merge(x2):
        last = p1[0] * p2[0]
print(last)