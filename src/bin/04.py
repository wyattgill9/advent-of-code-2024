import os

def read(file_path="input.txt"):
    with open(file_path) as f:
        return [line.strip() for line in f.readlines()]
    
def p1(grid):
    rows, cols = len(grid), len(grid[0])
    dirs = [(0,1), (0,-1), (1,0), (-1,0), (1,1), (1,-1), (-1,1), (-1,-1)]
    
    def valid(r, c, dx, dy):
        return (0 <= r + 3*dx < rows and 0 <= c + 3*dy < cols and
                all(grid[r+i*dx][c+i*dy] == letter for i, letter in enumerate('XMAS')))
    
    return sum(valid(i, j, dx, dy) 
              for i in range(rows) 
              for j in range(cols) 
              for dx, dy in dirs)

def p2(grid):
    rows, cols = len(grid), len(grid[0])
    
    def valid(r, c):
        if not (0 <= r-1 < rows and 0 <= r+1 < rows and 
                0 <= c-1 < cols and 0 <= c+1 < cols):
            return False
        if grid[r][c] != 'A':
            return False
        
        diags = [(grid[r-1][c-1], grid[r+1][c+1]),
                 (grid[r-1][c+1], grid[r+1][c-1])]
        return sum(pair in [('M','S'), ('S','M')] for pair in diags) == 2
    
    return sum(valid(i, j) for i in range(rows) for j in range(cols))

data = read()
print(f"Part 1: {p1(data)}\nPart 2: {p2(data)}")