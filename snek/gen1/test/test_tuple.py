from snek.gen1.lib.math.tuple import Tuple, Point, Vector


def test_point_constructor():
    p = Point(4, -4, 3)
    assert p == (4, -4, 3, 1.0)
    assert p.is_point()
    assert not p.is_vector()


def test_vector_constructor():
    v = Vector(4, -4, 3)
    assert v == (4, -4, 3, 0.0)
    assert not v.is_point()
    assert v.is_vector()


# Test with w=1.0 (a point)
def test_tuple_create_point():
    a = Tuple(4.3, -4.2, 3.1, 1.0)
    assert a[0] == 4.3
    assert a[1] == -4.2
    assert a[2] == 3.1
    assert a[3] == 1.0
    assert a.is_point()
    assert not a.is_vector()


# Test with w=0.0 (a vector)
def test_tuple_create_vector():
    a = Tuple(4.3, -4.2, 3.1, 0.0)
    assert a[0] == 4.3
    assert a[1] == -4.2
    assert a[2] == 3.1
    assert a[3] == 0.0
    assert not a.is_point()
    assert a.is_vector()


# Test add / sub vectors
def test_tuple_add():
    a = Tuple(3, -2, 5, 1)
    b = Tuple(-2, 3, 1, 0)
    c = a + b
    assert c == (1, 1, 6, 1)


def test_tuple_sub():
    p1 = Point(3, 2, 1)
    p2 = Point(5, 6, 7)
    ps = p1 - p2
    assert ps == (-2, -4, -6, 0.0)


def test_tuple_neg():
    a = Tuple(1, -2, 3, -4)
    assert a.neg() == (-1, 2, -3, 4)


def test_tuple_scalar_mul():
    a = Tuple(1, -2, 3, -4)
    a_m = a * 0.5
    a_execpected = Tuple(0.5, -1.0, 1.5, -2.0)
    assert a_m == a_execpected


def test_tuple_scalar_div():
    a = Tuple(1, -2, 3, -4)
    a_m = a / 2
    a_execpected = Tuple(0.5, -1.0, 1.5, -2.0)
    assert a_m == a_execpected


def test_tuple_magnitude():
    v = Vector(1, 2, 3)
    assert v.magnitude() == (14) ** 0.5
    v = Vector(-1, -2, -3)
    assert v.magnitude() == (14) ** 0.5
    v = Vector(0, 0, 0)
    assert v.magnitude() == 0.0
    v = Vector(1, 0, 0)
    assert v.magnitude() == 1.0
    v = Vector(0, 1, 0)
    assert v.magnitude() == 1.0
    v = Vector(0, 0, 1)
    assert v.magnitude() == 1.0
    v = Vector(1, 2, 3)
    assert v.magnitude() == (14) ** 0.5


def test_tuple_normalize():
    v = Vector(4, 0, 0)
    assert v.normalize() == Vector(1, 0, 0)
    v = Vector(1, 2, 3)
    norm = v.normalize()
    assert norm == Vector(1 / (14**0.5), 2 / (14**0.5), 3 / (14**0.5))
    assert abs(norm.magnitude() - 1.0) < 0.00001


def test_tuple_dot():
    a = Vector(1, 2, 3)
    b = Vector(2, 3, 4)
    assert a.dot(b) == 20


def test_tuple_cross():
    a = Vector(1, 2, 3)
    b = Vector(2, 3, 4)
    c_ab = a.cross(b)
    c_ba = b.cross(a)

    assert c_ab == Vector(-1, 2.0, -1)
    assert c_ba == Vector(1, -2, 1)
