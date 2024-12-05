import re

def solve_day3_p1(data):
    mul_pattern = r'mul\((\d{1,3}),(\d{1,3})\)'
    do_pattern = r'do\(\)'
    dont_pattern = r'don\'t\(\)'
    
    combined_pattern = f'(?P<mul>{mul_pattern})|(?P<do>{do_pattern})|(?P<dont>{dont_pattern})'
    matches = re.finditer(combined_pattern, data)
    
    total = 0
    enabled = True  
    
    for match in matches:
        if match.group('mul') and enabled:
            
            mul_match = re.match(mul_pattern, match.group('mul'))
            x, y = map(int, (mul_match.group(1), mul_match.group(2)))
            total += x * y
        elif match.group('do'):
            enabled = True
        elif match.group('dont'):
            enabled = False
    
    return total

def solve_day3_p2(data):
    mul_pattern = r'mul\((\d{1,3}),(\d{1,3})\)'
    do_pattern = r'do\(\)'
    dont_pattern = r'don\'t\(\)'
    
    combined_pattern = f'(?P<mul>{mul_pattern})|(?P<do>{do_pattern})|(?P<dont>{dont_pattern})'
    matches = re.finditer(combined_pattern, data)
    
    total = 0
    enabled = True  
    
    for match in matches:
        if match.group('mul') and not enabled:
            
            mul_match = re.match(mul_pattern, match.group('mul'))
            x, y = map(int, (mul_match.group(1), mul_match.group(2)))
            total += x * y
        elif match.group('do'):
            enabled = True
        elif match.group('dont'):
            enabled = False
    
    return total

if __name__ == "__main__":
    with open('../inputs/day3.txt', 'r') as f:
        data = f.read().strip()
    
    print(f"Part 1: {solve_day3_p1(data)}")
    print(f"Part 2: {solve_day3_p2(data)}")
