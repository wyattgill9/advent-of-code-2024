class Grid:
    def __init__(self, grid_data):
        self.grid = grid_data
        self.height = len(grid_data)
        self.width = len(grid_data[0]) if self.height > 0 else 0

    @classmethod
    def from_string(cls, raw, classify):
        grid_data = [[classify(char) for char in line] for line in raw.splitlines()]
        return cls(grid_data)

    def __getitem__(self, idx):
        return self.grid[idx]

    def __setitem__(self, idx, value):
        self.grid[idx] = value

def parse_raw(raw: str):
    grid = Grid.from_string(
        raw,
        classify=lambda i: {
            ".": 0,
            "#": 1,
            "^": 2,
            ">": 3,
            "v": 4,
            "<": 5,
        }[i],
    )
    for y, row in enumerate(grid):
        for x, val in enumerate(row):
            if val in (2, 3, 4, 5):
                start = (x, y), [[0, -1], [1, 0], [0, 1], [-1, 0]][val - 2]
                grid[y][x] = 0
                return grid, start
    return grid, ((0, 0), [1, 0])

def part_one(data):
    return len(collect_path(data))

def collect_path(data):
    grid, ((x, y), (dx, dy)) = data
    positions = set()
    while 0 <= x < grid.width and 0 <= y < grid.height:
        positions.add((x, y))
        nx, ny = x + dx, y + dy
        if 0 <= nx < grid.width and 0 <= ny < grid.height and grid[ny][nx] == 1:
            dx, dy = -dy, dx
        else:
            x, y = nx, ny
    return positions

def try_obstruction(data, ox=0, oy=0):
    grid, ((x, y), (dx, dy)) = data
    positions = set()
    while 0 <= x < grid.width and 0 <= y < grid.height:
        if (x, y, dx, dy) in positions:
            return True
        positions.add((x, y, dx, dy))
        nx, ny = x + dx, y + dy
        if (
            0 <= nx < grid.width
            and 0 <= ny < grid.height
            and (grid[ny][nx] == 1 or nx == ox and ny == oy)
        ):
            dx, dy = -dy, dx
        else:
            x, y = nx, ny
    return False

def part_two(data):
    ans = 0
    _, (pos, _) = data
    for x, y in collect_path(data):
        if (x, y) != pos and try_obstruction(data, x, y):
            ans += 1
    return ans

def read_input(file_path):
    with open(file_path, "r") as file:
        return file.read()

input_raw = read_input("../inputs/day6.txt")
data = parse_raw(input_raw)

print("Part One:", part_one(data))
print("Part Two:", part_two(data))
