import libfrustool
import math
from random import randint
import numpy
import time

vec = [1453478784, 2345234784565, 47846178946323, 4896778465]
vec2 = [3475455781, 234446456455, 3578678445664, 3458567564]
print(numpy.divide(vec, vec2))

vector = []
vector2 = []

start = time.time()
for x in range(10000):
    for x in range(100):
        vector.append(randint(1, 999999999999999))
        vector2.append(randint(1, 999999999999999))
    numpy.divide(vector, vector2)
stop = time.time()

print("Elapsed time for 10 million vector division using numpy: " + str(stop-start))