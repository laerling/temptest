#!/usr/bin/env python3

from PIL import Image
from math import sqrt
from random import random
import shutil
import subprocess
import sys
import tempfile


# get filename from arguments
if len(sys.argv) <= 2:
    print("Two args needed:")
    print("- Input file name")
    print("- Rounds")
    exit(1)

ifilename = sys.argv[1]
rounds = int(sys.argv[2])

# prepare initial image
tempdir = tempfile.TemporaryDirectory()
print(f"tempdir: {tempdir}")
def number_to_filename(num):
    return tempdir.name + "/{:05}.png".format(num)
shutil.copyfile(ifilename, number_to_filename(0))

# start outer loop
for num in range(0, rounds):

    # get filenames
    ifilename = number_to_filename(num)
    ofilename = number_to_filename(num + 1)

    # load image
    print("Reading from {}".format(ifilename))
    i = Image.open(ifilename)
    width = i.size[0]
    height = i.size[1]
    px = i.load()

    # change amount that the pixel travels
    x = 10/rounds*num # result will go from ~0.066 to ~0.99
    import math
    travel_fraction = 1/(1+math.e**-(x-5)) # sigmoid. I. e. accelerate with time
    print(travel_fraction)

    # do something
    for _ in range(width * height):

        # choose random pixel
        pos1 = (int(width * random()), int(height * random()))
        p1 = px[pos1[0], pos1[1]]

        # let the point tend to the three corners, according to RGB values
        pos2 = pos1
        corners = ((0, 0), (width - 1, 0), (0, height - 1), (width - 1, height - 1))
        pos2tendencies = []

        # influence of the three color channels
        for x in range(3):
            influence = p1[x] / 255
            pos2tendencies.append((
                pos2[0] - (influence * (pos2[0] - corners[x][0])),
                pos2[1] - (influence * (pos2[1] - corners[x][1]))
                ))

        # influence of brightness
        # Invert (1-) because we want the dark stuff to tend towards the
        # corner. Else it'll just cancel itself out with the other attractors.
        influence = 1 - ((p1[0] + p1[1] + p1[2]) / 3) / 255
        pos2tendencies.append((
            pos2[0] - (influence * (pos2[0] - corners[3][0])),
            pos2[1] - (influence * (pos2[1] - corners[3][1]))
            ))

        # calculate average between the altered points
        pos2 = (
                sum(map(lambda p: p[0], pos2tendencies)) // 4,
                sum(map(lambda p: p[1], pos2tendencies)) // 4
                )

        # move pixel a certain fraction of the distance towards the average
        pos2 = (
                pos1[0] + round((pos2[0] - pos1[0]) * travel_fraction),
                pos1[1] + round((pos2[1] - pos1[1]) * travel_fraction)
                )

        # switch pixels
        p2 = px[pos2[0], pos2[1]]
        px[pos1[0], pos1[1]] = p2
        px[pos2[0], pos2[1]] = p1

    # save to output file
    print("Saving to {}".format(ofilename))
    i.save(ofilename)

# make gif
print('Running: ffmpeg -y -i {}/%05d.png -vf fps=20 output.gif'.format(tempdir.name))
subprocess.run(['ffmpeg', '-y', '-i', tempdir.name + '/%05d.png', '-vf', 'fps=20', 'output.gif'])
