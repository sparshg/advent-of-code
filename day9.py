class Node:
    def __init__(self, head, track=False) -> None:
        self.x = 0
        self.y = 0
        self.h = head
        self.visited = set()
        self.track = track

    def update(self):
        if abs(self.h.x - self.x) <= 1 and abs(self.h.y - self.y) == 2:
            self.x = self.h.x
            self.y = (self.y + self.h.y) // 2
        elif abs(self.h.y - self.y) <= 1 and abs(self.h.x - self.x) == 2:
            self.y = self.h.y
            self.x = (self.x + self.h.x) // 2
        elif abs(self.h.y - self.y) == 2 and abs(self.h.x - self.x) == 2:
            self.x = (self.x + self.h.x) // 2
            self.y = (self.y + self.h.y) // 2
        if self.track:
            self.visited.add((self.x, self.y))


h = Node(head=None)

snake = [Node(head=h, track=True)]
for i in range(8):
    snake.append(Node(head=snake[i]))
snake[8].track = True


for line in open("input.txt", "r"):
    for _ in range(int(line[2:])):
        if "R" == line[0]:
            h.x += 1
        elif "L" == line[0]:
            h.x -= 1
        elif "U" == line[0]:
            h.y += 1
        elif "D" == line[0]:
            h.y -= 1
        for s in snake:
            s.update()

print(len(snake[0].visited), len(snake[8].visited))
