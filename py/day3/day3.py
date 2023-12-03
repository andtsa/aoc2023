lines = open("input.txt").readlines()
# lines = open("test1.txt").readlines()
# lines = open("test2.txt").readlines()


y = []
m = []
g = []

line_length = len(lines[0])

for l in range(len(lines)):
    line = lines[l]
    n = ''
    for c in range(len(line)):
        if line[c].isdigit():
            if len(n) < 1:
                s = c
            n += line[c]
        if not line[c].isdigit():
            if len(n) > 0:
                y.append((int(n), s, c - 1, l))
            n = ''
            s = 0
        if line[c] in ['!','#','\\','/','@','$','%','^','&','*','=','+','-','_','?']:
            m.append((c, l))
        if line[c] is '*':
            g.append((c, l))

total = []
print('y:', y)

print("lines", len(lines))

gears = {}

for n, s, e, l in y:
    added = False
    added_g = False
    for x in range(max(0, s-1), min(e+2, line_length)):
        for y in range(max(0, l-1), min(len(lines), l+2)):
            if (x, y) in m and not added:
                total.append(n)
                added = True
            if (x, y) in g and not added_g:
                if (x, y) in gears:
                    gears[(x, y)].append(n)
                else:
                    gears[(x, y)] = [n]
                added_g = True
            print(n, x, y)

ratios = 0

for gear, ns in gears.items():
    if len(ns) == 2:
        ratios += ns[0] * ns[1]

print(sum(total))
print(ratios)
"""
4361
"""