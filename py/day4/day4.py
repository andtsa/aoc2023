import re

lines = open("input.txt").readlines()
# lines = open("test1.txt").readlines()
# lines = open("test2.txt").readlines()

y = []
d = {}

t = int(re.findall(r"(\d+)", lines[-1])[0])
for i in range(1, t+1):
    d[i] = 1
for i in range(t+1, 2*t):
    d[i] = 0

d.setdefault(1)

for line in lines:
    p1 = line.split("|")[0]
    p1 = p1.split(":")[1]
    card_n = int(re.findall(r"(\d+)",line.split(":")[0])[0])
    p2 = line.split("|")[1]
    winning = re.findall(r" (\d+)", p1)
    n_winning = [int(x) for x in winning]
    own = re.findall(r" (\d+)", p2)
    n_own = [int(x) for x in own]
    total = 0
    for n in n_own:
        if n in n_winning:
            total += 1
    mult = d.get(card_n)
    for i in range(total):
        d[i+card_n+1] += mult
    print("card",card_n,total)
    # y.append(total)
print(d)
print(t)
s = 0
for i in range(1, t+1):
    s += d.get(i)

print(s)
