import heapq


with open("input.txt", "r") as fd:
    data = [
        [int(r) for r in line]
        for line in fd.read().splitlines()
    ]

SIZE = len(data) * 5

new_data = [[None for _ in range(SIZE)] for _ in range(SIZE)]
for y, row in enumerate(data):
    for x, r in enumerate(row):
        for i in range(5):
            for j in range(5):
                if r + i + j > 9:
                    new_data[y + i * SIZE // 5][x + j * SIZE // 5] = r + i + j - 9
                else:
                    new_data[y + i * SIZE // 5][x + j * SIZE // 5] = r + i + j

data = [
    [float("-inf"), *row, float("-inf")]
    for row in new_data
]
data = [[float("-inf")] * (SIZE + 2), *data, [float("-inf")] * (SIZE + 2)]

dist = {(y, x): float("inf") for y, row in enumerate(data) for x, _ in enumerate(row)}
dist[(1, 1)] = 0
pq = [(0, (1, 1))]
while pq:
    (u_cost, (uy, ux)) = heapq.heappop(pq)
    
    up = (uy - 1, ux)
    right = (uy, ux + 1)
    down = (uy + 1, ux)
    left = (uy, ux - 1)

    for vy, vx in (up, right, down, left):
        if {vy, vx} & {0, SIZE + 1}: continue

        v_cost = u_cost + data[vy][vx]
        if v_cost < dist[(vy, vx)]:
            dist[(vy, vx)] = v_cost
            heapq.heappush(pq, (v_cost, (vy, vx)))

print(dist[(len(data) - 2, len(data[0]) - 2)])