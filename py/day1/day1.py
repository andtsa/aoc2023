import re

lines = open("input.txt").readlines()
y = []

r = {
 "one":1, "two":2, "three":3, "four":4, "five":5, "six":6, "seven":7, "eight":8, "nine":9
}


for line in lines:
    l = line
    for key, val in r.items():
        l = l.replace(key,key+str(val)+key)
    n = re.findall(r'\d', l)
    y.append(int(n[0]+n[-1]))
    print(y)

print(sum(y))
