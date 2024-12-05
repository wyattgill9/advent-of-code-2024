def is_safe_report(report):
    # Check if sequence is monotonic (only increasing or only decreasing)
    diffs = [report[i] - report[i-1] for i in range(1, len(report))]
    is_monotonic = all(d >= 0 for d in diffs) or all(d <= 0 for d in diffs)
    
    # Check if all differences are between 1 and 3
    valid_diffs = all(1 <= abs(d) <= 3 for d in diffs)
    
    return is_monotonic and valid_diffs

def count_safe_reports(input_data, allow_removal=False):
    safe_count = 0
    for line in input_data:
        report = list(map(int, line.split()))
        
        # check original report
        if is_safe_report(report):
            safe_count += 1
            continue
            
        #  try removing one number
        if allow_removal:
            if any(is_safe_report(report[:i] + report[i+1:]) for i in range(len(report))):
                safe_count += 1
                
    return safe_count

# Read input and process both parts
with open('../inputs/day2.txt', 'r') as file:
    input_data = [line.strip() for line in file.readlines()]

print(f"Part 1: {count_safe_reports(input_data)}")
print(f"Part 2: {count_safe_reports(input_data, allow_removal=True)}")
