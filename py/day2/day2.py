import re

"""
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"""

# PART = 1
PART = 2

lines = open("input.txt").readlines()
# lines = open("test1.txt").readlines()
# lines = open("test2.txt").readlines()
y = []
n_blue = 14
n_red = 12
n_green = 13

for line in lines:
    game_number = int(line.split(":")[0].split(" ")[1])
    # print(re.findall(r'(\d) blue', line))
    max_blue = max([int(x) for x in re.findall(r'(\d+) blue', line)])
    max_red = max([int(x) for x in re.findall(r'(\d+) red', line)])
    max_green = max([int(x) for x in re.findall(r'(\d+) green', line)])
    # print(game_number, max_blue,max_red,max_green)
    if PART == 1:
        if max_green <= n_green and max_blue <= n_blue and max_red <= n_red:
            y.append(game_number)
    elif PART == 2:
        y.append(max_green * max_red * max_blue)

print(sum(y))
"""
only 12 red cubes, 13 green cubes, and 14 blue cubes
"""