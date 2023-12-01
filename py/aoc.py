import time
import inspect
from os.path import splitext, basename
from pathlib import Path
import requests
from datetime import date
from mpl_toolkits import mplot3d
import numpy as np
import matplotlib.pyplot as plt

root = Path(__file__).parents[0]
year = str(date.today())[:4]


def download_input(day):
    d = str(day).zfill(2)
    Path(root, f'day{d}').mkdir(exist_ok=True)
    url = f'https://adventofcode.com/{year}/day/{day}/input'
    req = requests.get(url, cookies={'session': Path(root, f'external/session_token.txt').read_text().strip()})
    Path(root, f'day{d}/day{d}.txt').write_bytes(req.content[:-2])
    url = f'https://adventofcode.com/{year}/day/{day}'
    req = requests.get(url)
    test_input = req.text.split("example")[1].split("<pre><code>")[1].split("</code></pre>")[0]
    Path(root, f'day{d}/day{d}test.txt').write_text(test_input)


def newDay():
    day = int(str(date.today())[8:10])
    d = str(day).zfill(2)
    Path(root, f'day{d}').mkdir(exist_ok=True)
    try:
        f = open(f'day{d}/day{d}.py')
        print('file already exists:', f)
        f.close()
    except FileNotFoundError:
        f = open(f'day{d}/day{d}.py', 'a')
        template = open(str(root)+'/template.txt').read()
        f.write(template)
        print('file created!', f)
        f.close()
    download_input(day)


def printGrid(grid):
    for g in grid:
        for d in g:
            print(d, end=" ")
        print(end="\n")
    print(end="\n")


def printGrid3D(grid):
    fig = plt.figure()
    ax = fig.add_subplot(211, projection='3d')
    xdata = ydata = zdata = []
    for x in range(len(grid)):
        for y in range(len(grid[x])):
            for z in range(len(grid[x][y])):
                if grid[x][y][z] == 1:
                    xdata.append(x)
                    ydata.append(y)
                    zdata.append(z)
    ax.plot(xdata, ydata, zdata, 's')
    ax.grid(True)
    ax.set_title('3d grid')
    plt.show()


def hasDuplicates(x):
    k = 0
    for l in range(len(x)-1):
        k=l+1
        for l in range(k):
            if x[l]==x[k]:
                return True
    return False


def removeDuplicates(l):
     return list(dict.fromkeys(l))


def tdist(p1, p2):
    return abs(p2[0]-p1[0]) + abs(p2[1]-p1[1])


def init(day, mode):
    infile = "day" + str(day) + mode * "test" + ".txt"
    text = open(infile).read().strip()
    lines = [x for x in text.split('\n')]
    t0 = time.time_ns()
    return lines, t0


def dt(t0):
    print("\ndt: " + str((time.time_ns()-t0)/10**6) + "ms")


# newDay()