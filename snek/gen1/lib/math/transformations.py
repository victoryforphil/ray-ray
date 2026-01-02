from math import cos, sin

from snek.gen1.lib.math.matrix import IdentityMatrix4x4, Matrix
from snek.gen1.lib.math.tuple import Point, Tuple, Vector


def Translation(position: Tuple):
    # [1, 0, 0, x]
    # [0, 1, 0, y]
    # [0, 0, 1, z]
    # [0, 0, 0, 1]
    x = position.x
    y = position.y
    z = position.z
    return Matrix(
        data=[
            [1, 0, 0, x],
            [0, 1, 0, y],
            [0, 0, 1, z],
            [0, 0, 0, 1],
        ]
    )


def Scale(scale: Tuple):
    # [x, 0, 0, 0]
    # [0, y, 0, 0]
    # [0, 0, z, 0]
    # [0, 0, 0, 1]
    x = scale.x
    y = scale.y
    z = scale.z
    return Matrix(
        data=[
            [x, 0, 0, 0],
            [0, y, 0, 0],
            [0, 0, z, 0],
            [0, 0, 0, 1],
        ]
    )


def Reflect():
    return Scale(Vector(-1.0, 1.0, 1.0))


def RotationX(r):
    return Matrix(
        data=[
            [1, 0, 0, 0],
            [0, cos(r), -sin(r), 0],
            [0, sin(r), cos(r), 0],
            [0, 0, 0, 1],
        ]
    )


def RotationY(r):
    return Matrix(
        data=[
            [cos(r), 0, sin(r), 0],
            [0, 1, 0, 0],
            [-sin(r), 0, cos(r), 0],
            [0, 0, 0, 1],
        ]
    )


def RotationZ(r):
    return Matrix(
        data=[
            [cos(r), -sin(r), 0, 0],
            [sin(r), cos(r), 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1],
        ]
    )
