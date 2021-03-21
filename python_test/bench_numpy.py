import libfrustool
import math
from random import randint
import numpy
import time

vec = [1453478784, 2345234784565, 47846178946323, 4896778465]
vec2 = [3475455781, 234446456455, 3578678445664, 3458567564]
print(numpy.divide(vec, vec2))

start = time.time()
for x in range(10000000):
    for x in range(4):
        vec[x] = randint(1, 999999999999999)
        vec2[x] = randint(1, 999999999999999)
    numpy.divide(vec, vec2)
stop = time.time()

print("Elapsed time for 10 million vector division using numpy: " + str(stop-start))