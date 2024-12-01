# Part 1:

left_list = []
right_list = []

with open('input.txt', 'r') as file:
    for line in file:
        left, right = line.strip().split()
        left_list.append(int(left))
        right_list.append(int(right))


left_list.sort()
right_list.sort()

total_distance = sum(abs(left - right) for left, right in zip(left_list, right_list))

print(f"Total distance: {total_distance}")

# Part 2:

left_list = []
right_list = []

with open('input.txt', 'r') as file:
    for line in file:
        left, right = line.strip().split()
        left_list.append(int(left))
        right_list.append(int(right))

right_counts = {}
for num in right_list:
    right_counts[num] = right_counts.get(num, 0) + 1

similarity_score = sum(num * right_counts.get(num, 0) for num in left_list)

print(f"Similarity score: {similarity_score}")