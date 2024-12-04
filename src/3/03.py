import re

def solve_day3(input_file="input.txt"):
    with open(input_file, 'r') as f:
        data = f.read().strip()
    
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

if __name__ == "__main__":
    result = solve_day3()
    print(f"{result}")
