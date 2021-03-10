#!/usr/bin/env python3

from PIL import Image
from math import sqrt
from random import random
import sys


# get filenames from arguments
if len(sys.argv) <= 1:
    exit(1)

ifilename = sys.argv[1]
if len(sys.argv) <= 2:
    ofilename = "./result.png"
else:
    ofilename = sys.argv[2]

# load image
print("Reading from {}".format(ifilename))
i = Image.open(ifilename)
width = i.size[0]
height = i.size[1]
px = i.load()

# do something
for _ in range(width * height):

    # choose random pixel
    pos1 = (int(width * random()), int(height * random()))
    p1 = px[pos1[0], pos1[1]]

    # let the point tend to the three corners, according to RGB values
    pos2 = pos1
    corners = ((0, 0), (width - 1, 0), (width // 2, height - 1))
    for x in range(3):
        influence = p1[x] / 255
        pos2 = (pos2[0] - (influence * (pos2[0] - corners[x][0])),
                pos2[1] - (influence * (pos2[1] - corners[x][1])))

    # normalize coordinates
    pos2 = (int(pos2[0]), int(pos2[1]))

    # switch pixels
    p2 = px[pos2[0], pos2[1]]
    px[pos1[0], pos1[1]] = p2
    px[pos2[0], pos2[1]] = p1

# save to output file
print("Saving to {}".format(ofilename))
i.save(ofilename)
