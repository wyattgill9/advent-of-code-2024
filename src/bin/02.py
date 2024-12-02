def is_safe_report(report):
    is_increasing = True
    is_decreasing = True
    for i in range(1, len(report)):
        if report[i] > report[i-1]:
            is_decreasing = False
        elif report[i] < report[i-1]:
            is_increasing = False

    if not is_increasing and not is_decreasing:
        return False

    for i in range(1, len(report)):
        if not (1 <= abs(report[i] - report[i-1]) <= 3):
            return False

    return True

def can_be_safe_by_removing_one(report):
    for i in range(len(report)):
        modified_report = report[:i] + report[i+1:]
        if is_safe_report(modified_report):
            return True
    return False

def count_safe_reports(input_data):
    safe_count = 0
    for line in input_data:
        report = list(map(int, line.split()))
        if is_safe_report(report) or can_be_safe_by_removing_one(report):
            safe_count += 1
    return safe_count

with open('../../input.txt', 'r') as file:
    input_data = file.readlines()

input_data = [line.strip() for line in input_data]

print(count_safe_reports(input_data))
