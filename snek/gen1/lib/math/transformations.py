from snek.gen1.lib.math.matrix import Matrix, IdentityMatrix4x4
from snek.gen1.lib.math.tuple import Tuple, Vector, Point

def Translation(position: Tuple):
    # [1, 0, 0, x]
    # [0, 1, 0, y]
    # [0, 0, 1, z]
    # [0, 0, 0, 1]
    x = position.x
    y = position.y 
    z = position.z
    return Matrix(data=[
        [1, 0, 0, x],
        [0, 1, 0, y],
        [0, 0, 1, z],
        [0, 0, 0, 1],
    ])