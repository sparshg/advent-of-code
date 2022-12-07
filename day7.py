class Dir:
    p1 = 0
    p2 = 70000000
    required = 0

    def __init__(self, name, parent=None, dirs=None, files=0) -> None:
        self.name = name
        self.dirs = [] if dirs is None else dirs
        self.files = files
        self.parent = parent

    def size(self):
        size = self.files + sum([x.size() for x in self.dirs])
        if size <= 100000: Dir.p1 += size
        return size
    
    def size2(self):
        size = self.files + sum([x.size2() for x in self.dirs])
        if size - Dir.required >= 0:
            Dir.p2 = min(Dir.p2, size - Dir.required)
        return size
        
curr = None
root = None
for line in open('input.txt', 'r'):
    line = line.strip()
    if line == '$ cd /':
        curr = Dir('/')
        root = curr
    elif line == '$ cd ..':
        curr = curr.parent
    elif line[:4] == '$ cd':
        curr = next((x for x in curr.dirs if x.name == line[5:]), None)
    elif line == '$ ls':
        continue
    elif line[:3] == 'dir':
        curr.dirs.append(Dir(line[4:], curr))
    else:
        curr.files += int(line.split(' ')[0])
        
Dir.required = root.size() - 40000000
print(Dir.p1)
root.size2()
print(Dir.p2 + Dir.required)
