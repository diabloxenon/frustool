from random import randint
import libfrustool as ta
import math
import time
# PI = 3.14159276
PI = 3.14159265358979323846264338327950288419716939937510


def deg2rad(angle):
    return angle * PI / 180

ta.rocr100([1,2,3,4], 2)
ta.rocp([1, 2, 3, 4], 2)

# sin_90 = ta.sin([deg2rad(60), deg2rad(90), deg2rad(45), deg2rad(30)])
# cotan_90 = 1/ta.tan(deg2rad(90))
# tanh_90 = ta.tanh(deg2rad(90))
# asin_90 = ta.asin(deg2rad(90))

# print(sin_90, cotan_90, tanh_90, asin_90)


# vec = [1453478784, 1, 47846178946323, 4896778465]
# vec2 = [3475455781, 0, 3578678445664, 3458567564]

# print(ta.ln(vec))

# print(ta.div(vec, vec2))

vector = []
vector2 = []

start = time.time()
for x in range(10000):
    for x in range(100):
        vector.append(randint(1, 999999999999999))
        vector2.append(randint(1, 999999999999999))
    ta.div(vector, vector2)
stop = time.time()

print("Elapsed time for 10 million vector division using frustool: " + str(stop-start))
