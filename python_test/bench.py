from random import randint
import libfrustool
import math
import time
PI = 3.14159278


def deg2rad(angle):
    return 90 * PI / 180


sin_90 = libfrustool.sin(deg2rad(90))
cotan_90 = 1/libfrustool.tan(deg2rad(90))
tanh_90 = libfrustool.tanh(deg2rad(90))
asin_90 = libfrustool.asin(deg2rad(90))

print(sin_90, cotan_90, tanh_90, asin_90)


vec = [1453478784, 1, 47846178946323, 4896778465]
vec2 = [3475455781, 0, 3578678445664, 3458567564]

print(libfrustool.div(vec, vec2))

start = time.time()
for x in range(10000000):
    for x in range(4):
        vec[x] = randint(1, 999999999999999)
        vec2[x] = randint(1, 999999999999999)
    libfrustool.div(vec, vec2)
stop = time.time()

print("Elapsed time for 10 million vector division using frustool: " + str(stop-start))
