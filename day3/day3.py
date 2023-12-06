from collections import defaultdict


maze = open("input.txt", "r").read().split("\n")
width = len(maze[0])
height = len(maze)

id_ = 0
id_to_value = {}
pos_to_id = {}
valid_ids = set()

for i in range(height):
    j = 0
    while j < width:
        if maze[i][j] == ".":
            j += 1
            continue
        if maze[i][j] >= "0" and maze[i][j] <= "9":
            id_ += 1
            value = 0
            while j < width and maze[i][j] >= "0" and maze[i][j] <= "9":
                value = value * 10 + int(maze[i][j])
                pos_to_id[(i, j)] = id_
                j += 1
            id_to_value[id_] = value
        j += 1

star_id = 0
star_id_number_ids = defaultdict(set)

for i in range(height):
    for j in range(width):
        if maze[i][j] == ".":
            continue
        if maze[i][j] >= "0" and maze[i][j] <= "9":
            continue
        # symbol
        # for part1: mark 8 neighbors as valid
        # for part2: if symbol is '*', mark neighbors as same star id
        if maze[i][j] == "*":
            star_id += 1
        for di in range(-1, 2):
            for dj in range(-1, 2):
                if di == 0 and dj == 0:
                    continue
                if i + di < 0 or i + di >= height or j + dj < 0 or j + dj >= width:
                    continue
                if maze[i + di][j + dj] == ".":
                    continue
                valid_ids.add(pos_to_id[(i + di, j + dj)])
                if maze[i][j] == "*":
                    star_id_number_ids[star_id].add(pos_to_id[(i + di, j + dj)])

part1_ans = 0
for id_ in valid_ids:
    part1_ans += id_to_value[id_]

part2_ans = 0
for star_id, number_ids in star_id_number_ids.items():
    if len(number_ids) == 2:
        part2_ans += id_to_value[list(number_ids)[0]] * id_to_value[list(number_ids)[1]]

print("part1:", part1_ans)
print("part2:", part2_ans)
