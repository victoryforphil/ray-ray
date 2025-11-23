from snek.gen1.lib.math.matrix import Matrix, IdentityMatrix4x4
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

def test_matix_4x4_mul():
    ma = Matrix(data = [
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 8., 7., 6.],
        [5., 4., 3., 2.],
    ])
    mb = Matrix(data = [
        [-2., 1., 2., 3.],      
        [3., 2., 1., -1.],
        [4., 3., 6., 5.],
        [1., 2., 7., 8.],
    ])
    m_expected = Matrix(data = [
        [20., 22., 50., 48.],
        [44., 54., 114., 108.],
        [40., 58., 110., 102.],
        [16., 26., 46., 42.],
    ])
    assert ma * mb == m_expected


def test_matrix_4x4_mul_tuple():
    m = Matrix(data = [
        [1., 2., 3., 4.],
        [2., 4., 4., 2.],
        [8., 6., 4., 1.],
        [0., 0., 0., 1.],
    ])
    t = (1., 2., 3., 1.)

    result = m * t

    expected = Matrix(data = [
        [18.],
        [24.],
        [33.],
        [1.],
    ])

    assert result == expected   


def test_matrix_4x4identity_mul():
    m = Matrix(data = [
        [0., 1., 2., 4.],
        [1., 2., 4., 8.],
        [2., 4., 8., 16.],
        [4., 8., 16., 32.],
    ])

    identity = IdentityMatrix4x4()
    assert m * identity == m


def test_tuple_identity_mul():
    t = (1.,2.,3.,4.)
    result = IdentityMatrix4x4() * t
    expected = Matrix(data = [
        [1.],
        [2.],
        [3.],
        [4.],
    ])
    assert result == expected


def test_matrix_transpose():
    m = Matrix(data = [
        [0., 9., 3., 0.],
        [9., 8., 0., 8.],
        [1., 8., 5., 3.],
        [0., 0., 5., 8.],
    ])

    m_transposed = Matrix(data = [
        [0., 9., 1., 0.],
        [9., 8., 8., 0.],
        [3., 0., 5., 5.],
        [0., 8., 3., 8.],
    ])

    assert m.data != m_transposed.data
    assert m.data == m_transposed.transpose().data

def test_matrix_identity_transpose():
    identity = IdentityMatrix4x4()
    assert identity.transpose() == identity


def test_matrix_determinant_2x2():
    m = Matrix([
        [1.0, 5.0],
        [-3.0, 2.0]
    ])

    d = m.determinant()
    assert d == 17.

def test_matrix_submatrix_3x3():
    m3 = Matrix([
        [1., 5., 0.],
        [-3., 2., 7.],
        [0., 6., -3.]
    ])

    m2 = Matrix([
        [-3., 2.],
        [0., 6.]
    ])

    ma = m3.submatrix(0,2)
  
    assert ma == m2
    assert ma.width == 2
    assert ma.height == 2


def test_matrix_submatrix_4x4():
    m4 = Matrix([
        [-6., 1., 1., 6.],
        [-8., 5., 8., 6.],
        [-1., 0., 8., 2.],
        [-7., 1., -1., 1.],
    ])

    m3 = Matrix([
        [-6., 1. , 6.],
        [-8., 8. , 6.],
        [-7., -1. , 1.]
    ])

    m_sub = m4.submatrix(2, 1)
    assert m_sub == m3
    assert m_sub.width == 3
    assert m_sub.height == 3