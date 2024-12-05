import os

def read_input(file):
    return [line.strip() for line in open(file)]

def p1():
    rules = []
    lists = []
    reading_rules = True
    
    for line in data:
        if not line:
            reading_rules = False
            continue
            
        if reading_rules:
            a, b = line.split('|')
            rules.append((int(a), int(b)))
        else:
            lists.append([int(x) for x in line.split(',')])
    
    total = 0
    for nums in lists:
        if all(nums.index(a) < nums.index(b) for a,b in rules if a in nums and b in nums):
            total += nums[len(nums)//2]
    return total

def p2():
    rules = []
    lists = []
    reading_rules = True
    
    for line in data:
        if not line:
            reading_rules = False
            continue
            
        if reading_rules:
            a, b = line.split('|')
            rules.append((int(a), int(b)))
        else:
            lists.append([int(x) for x in line.split(',')])
    
    total = 0
    for nums in lists:
        if not all(nums.index(a) < nums.index(b) for a,b in rules if a in nums and b in nums):
            graph = {x: set() for x in nums}
            for a,b in rules:
                if a in nums and b in nums:
                    graph[a].add(b)
                    
            order = []
            seen = set()
            
            def visit(n):
                if n in seen:
                    return
                seen.add(n)
                for next in graph[n]:
                    visit(next)
                order.append(n)
                
            for n in nums:
                if n not in seen:
                    visit(n)
                    
            order.reverse()
            total += order[len(order)//2]
            
    return total

data = read_input("../inputs/day5.txt")
print(f"Part 1: {p1()}\nPart 2: {p2()}")