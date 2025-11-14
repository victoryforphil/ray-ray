from snek.gen1.lib.math.matrix import Matrix
def test_matrix_4x4_new():
    m  = Matrix(data = [
        [1., 2., 3., 4.],
        [5.5, 6.5, 7.5, 8.5],
        [9., 10., 11., 12.],
        [13.5, 14.5, 15.5, 16.5],
    ])

    assert m[(0,0)] == 1.0
    assert m[(0,3)] == 4.
    assert m[(1,0)] == 5.5
    assert m[(1,2)] == 7.5
    assert m[(2,2)] == 11.0
    assert m[(3,0)] == 13.5
    assert m[(3,2)] == 15.5


def test_matrix_2x2_new():
    m = Matrix(data = [
        [-3., 5.], 
        [1., -2.]
    ])

    assert m[(0,0)] == -3.0
    assert m[(0,1)] == 5.0
    assert m[(1,0)] == 1.0
    assert m[(1,1)] == -2.0

def test_matrix_3x3_new():
    m = Matrix(data = [
        [-3., 5., 0.], 
        [1., -2., -7.0],
        [0., 1., 1.]
    ])

    assert m[(0,0)] == -3.0
    assert m[(0,1)] == 5.0
    assert m[(0,2)] == 0.0
    assert m[(1,0)] == 1.0
    assert m[(1,1)] == -2.0


def test_matrix_equality():
    m1 = Matrix(data = [
        [1., 2., 3., 4.],
        [5.5, 6.5, 7.5, 8.5],
        [9., 10., 11., 12.],
        [13.5, 14.5, 15.5, 16.5],
    ])
    m2 = Matrix(data = [
        [1., 2., 3., 4.],
        [5.5, 6.5, 7.5, 8.5],
        [9., 10., 11., 12.],
        [13.5, 14.5, 15.5, 16.5],
    ])
    assert m1 == m2

def test_matrix_inequality():
    m1 = Matrix(data = [
        [1., 2., 3., 4.],
        [5.5, 6.5, 7.5, 8.5],
        [9., 10., 11., 12.],
        [13.5, 14.5, 15.5, 16.5],
    ])
    m2 = Matrix(data = [
        [1., 3., 3., 4.],
        [5.5, 6.5, 7.5, 8.5],
        [9., 10., 11., 12.],
        [13.5, 14.5, 15.5, 16.5],
    ])
    assert m1 != m2
 