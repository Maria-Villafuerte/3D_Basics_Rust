# maze.py
import json
from random import shuffle, randrange

def make_maze(w=16, h=8, output_format='text'):
    vis = [[0] * w + [1] for _ in range(h)] + [[1] * (w + 1)]
    ver = [["|  "] * w + ['|'] for _ in range(h)] + [[]]
    hor = [["+--"] * w + ['+'] for _ in range(h + 1)]

    def walk(x, y):
        vis[y][x] = 1

        d = [(x - 1, y), (x, y + 1), (x + 1, y), (x, y - 1)]
        shuffle(d)
        for (xx, yy) in d:
            if vis[yy][xx]: continue
            if xx == x: hor[max(y, yy)][x] = "+  "
            if yy == y: ver[y][max(x, xx)] = "   "
            walk(xx, yy)

    walk(randrange(w), randrange(h))

    if output_format == 'text':
        s = ""
        for (a, b) in zip(hor, ver):
            s += ''.join(a + ['\n'] + b + ['\n'])
        l = list(s)
        l[w * 3 + 3] = 'p'
        l[((w * 3 + 3) * -1) - 3] = 'g'
        return "".join(l)
    if output_format == 'json':
        s = ""
        for (a, b) in zip(hor, ver):
            s += ''.join(a + ['\n'] + b + ['\n'])

        jsona = []
        for row in s.split('\n'):
            if row:
                jsona.append(list(row))

        jsona[1][1] = 'p'
        jsona[-2][-2] = 'g'
        return json.dumps(jsona)

# Test the function
print(make_maze(5, 5, 'text'))
