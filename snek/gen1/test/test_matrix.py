from snek.gen1.lib.math.matrix import Matrix, IdentityMatrix4x4


def test_matrix_4x4_new():
    m = Matrix(
        data=[
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]
    )

    assert m[(0, 0)] == 1.0
    assert m[(0, 3)] == 4.0
    assert m[(1, 0)] == 5.5
    assert m[(1, 2)] == 7.5
    assert m[(2, 2)] == 11.0
    assert m[(3, 0)] == 13.5
    assert m[(3, 2)] == 15.5


def test_matrix_2x2_new():
    m = Matrix(data=[[-3.0, 5.0], [1.0, -2.0]])

    assert m[(0, 0)] == -3.0
    assert m[(0, 1)] == 5.0
    assert m[(1, 0)] == 1.0
    assert m[(1, 1)] == -2.0


def test_matrix_3x3_new():
    m = Matrix(data=[[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]])

    assert m[(0, 0)] == -3.0
    assert m[(0, 1)] == 5.0
    assert m[(0, 2)] == 0.0
    assert m[(1, 0)] == 1.0
    assert m[(1, 1)] == -2.0


def test_matrix_equality():
    m1 = Matrix(
        data=[
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]
    )
    m2 = Matrix(
        data=[
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]
    )
    assert m1 == m2


def test_matrix_inequality():
    m1 = Matrix(
        data=[
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]
    )
    m2 = Matrix(
        data=[
            [1.0, 3.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]
    )
    assert m1 != m2


def test_matix_4x4_mul():
    ma = Matrix(
        data=[
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]
    )
    mb = Matrix(
        data=[
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]
    )
    m_expected = Matrix(
        data=[
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]
    )
    assert ma * mb == m_expected


def test_matrix_4x4_mul_tuple():
    m = Matrix(
        data=[
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    )
    t = (1.0, 2.0, 3.0, 1.0)

    result = m * t

    expected = Matrix(
        data=[
            [18.0],
            [24.0],
            [33.0],
            [1.0],
        ]
    )

    assert result == expected


def test_matrix_4x4identity_mul():
    m = Matrix(
        data=[
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]
    )

    identity = IdentityMatrix4x4()
    assert m * identity == m


def test_tuple_identity_mul():
    t = (1.0, 2.0, 3.0, 4.0)
    result = IdentityMatrix4x4() * t
    expected = Matrix(
        data=[
            [1.0],
            [2.0],
            [3.0],
            [4.0],
        ]
    )
    assert result == expected


def test_matrix_transpose():
    m = Matrix(
        data=[
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]
    )

    m_transposed = Matrix(
        data=[
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]
    )

    assert m.data != m_transposed.data
    assert m.data == m_transposed.transpose().data


def test_matrix_identity_transpose():
    identity = IdentityMatrix4x4()
    assert identity.transpose() == identity


def test_matrix_determinant_2x2():
    m = Matrix([[1.0, 5.0], [-3.0, 2.0]])

    d = m.determinant()
    assert d == 17.0


def test_matrix_submatrix_3x3():
    m3 = Matrix([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]])

    m2 = Matrix([[-3.0, 2.0], [0.0, 6.0]])

    ma = m3.submatrix(0, 2)

    assert ma == m2
    assert ma.width == 2
    assert ma.height == 2


def test_matrix_submatrix_4x4():
    m4 = Matrix(
        [
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]
    )

    m3 = Matrix([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]])

    m_sub = m4.submatrix(2, 1)
    assert m_sub == m3
    assert m_sub.width == 3
    assert m_sub.height == 3


def test_matrix_minor_3x3():
    ma = Matrix(
        [
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0],
        ]
    )

    mb = ma.submatrix(1, 0)
    md = mb.determinant()

    assert md == 25

    # --

    m_minor = ma.minor(1, 0)

    assert m_minor == 25


def test_matrix_cofactor_3x3():
    ma = Matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]])

    minor_1 = ma.minor(0, 0)
    assert minor_1 == -12.0

    cofactor_1 = ma.cofactor(0, 0)
    assert cofactor_1 == -12.0

    minor_2 = ma.minor(1, 0)
    assert minor_2 == 25.0

    cofactor_2 = ma.cofactor(1, 0)
    assert cofactor_2 == -25.0


def test_matrix_determinant_3x3():
    ma = Matrix([
        [1., 2., 6.],
        [-5., 8., -4.],
        [2., 6., 4.]
    ])

    assert ma.cofactor(0,0) == 56
    assert ma.cofactor(0,1) == 12
    assert ma.cofactor(0,2) == -46

    assert ma.determinant() == -196
    

def test_matrix_determinant_4x4():

    ma = Matrix([
        [-2, -8, 3, 5],
        [-3, 1, 7, 3],
        [1, 2, -9, 6],
        [-6, 7, 7, -9]
    ])

    assert ma.cofactor(0,0) == 690
    assert ma.cofactor(0,1) == 447
    assert ma.cofactor(0,2) == 210
    assert ma.cofactor(0,3) == 51

    assert ma.determinant() == -4071