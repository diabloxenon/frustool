import libfrustool
import math

PI = 3.14159278


def deg2rad(angle):
    return 90 * PI / 180


sin_90 = libfrustool.sin(deg2rad(90))

cotan_90 = 1/libfrustool.tan(deg2rad(90))

tanh_90 = libfrustool.tanh(deg2rad(90))

asin_90 = libfrustool.asin(deg2rad(90))

print(sin_90, cotan_90, tanh_90, asin_90)