import math

from snek.gen1.lib.math.transformations import (
    Reflect,
    RotationX,
    RotationY,
    RotationZ,
    Scale,
    Shearing,
    Translation,
)
from snek.gen1.lib.math.tuple import Point, Vector


def test_transform_translation_matrix_mul():
    transform = Translation(Vector(5.0, -3.0, 2.0))
    p = Point(-3.0, 4.0, 5.0)

    transformed_p = transform * p
    expected_p = Point(2.0, 1.0, 7.0)
    assert transformed_p.as_tuple() == expected_p


def test_transform_inverse_translate_mul():
    transform = Translation(Vector(5.0, -3.0, 2.0))
    inv = transform.inverse()
    p = Point(-3.0, 4.0, 5.0)

    transformed_p = inv * p
    expected_p = Point(-8.0, 7.0, 3.0)
    assert transformed_p.as_tuple() == expected_p


def test_tranform_translate_unchanged_vectors():
    transform = Translation(Vector(5.0, -3.0, 2.0))
    v = Vector(-3.0, 4.0, 5.0)
    tv = transform * v
    assert tv.as_tuple() == v


# Scaling


def test_scale_point():
    transform = Scale(Vector(2.0, 3.0, 4.0))
    p = Point(-4.0, 6.0, 8.0)
    pt = transform * p

    assert pt.as_tuple() == Point(-8.0, 18.0, 32.0)


def test_scale_vector():
    transform = Scale(Vector(2.0, 3.0, 4.0))
    v = Vector(-4.0, 6.0, 8.0)
    vt = transform * v

    assert vt.as_tuple() == Vector(-8.0, 18.0, 32.0)


def test_scale_inverse():
    transform = Scale(Vector(2.0, 3.0, 4.0))
    inv_transform = transform.inverse()
    v = Vector(-4.0, 6.0, 8.0)
    vt = inv_transform * v

    assert vt.as_tuple() == Vector(-2.0, 2.0, 2.0)


def test_scale_reflect():
    transform = Reflect()

    p = Point(2.0, 3.0, 4.0)
    rp = transform * p

    assert rp.as_tuple() == Point(-2.0, 3.0, 4.0)


def test_rotate_x_axis():
    pi = math.pi
    p = Point(0.0, 1.0, 0.0)
    half_quater = RotationX(pi / 4.0)
    full_quater = RotationX(pi / 2.0)

    half_p = half_quater * p
    assert half_p == Point(0.0, math.sqrt(2.0) / 2.0, math.sqrt(2.0) / 2.0)
    full_p = full_quater * p
    assert full_p == Point(0.0, 0.0, 1.0)


def test_rotate_x_axis_inverse():
    pi = math.pi
    p = Point(0.0, 1.0, 0.0)
    half_quater = RotationX(pi / 4.0)
    inv = half_quater.inverse()

    half_p = inv * p
    assert half_p == Point(0.0, math.sqrt(2.0) / 2.0, -math.sqrt(2.0) / 2.0)


def test_rotate_y_axis():
    pi = math.pi
    p = Point(0.0, 0.0, 1.0)
    half_quater = RotationY(pi / 4.0)
    full_quater = RotationY(pi / 2.0)

    half_p = half_quater * p
    assert half_p == Point(math.sqrt(2.0) / 2.0, 0.0, math.sqrt(2.0) / 2.0)
    full_p = full_quater * p
    assert full_p == Point(1.0, 0.0, 0.0)


def test_rotate_z_axis():
    pi = math.pi
    p = Point(0.0, 1.0, 0.0)
    half_quater = RotationZ(pi / 4.0)
    full_quater = RotationZ(pi / 2.0)

    half_p = half_quater * p
    assert half_p == Point(-math.sqrt(2.0) / 2.0, math.sqrt(2.0) / 2.0, 0.0)
    full_p = full_quater * p
    assert full_p == Point(-1.0, 0.0, 0.0)


def test_sheer_x_y():
    transform = Shearing(1, 0, 0, 0, 0, 0)
    p = Point(2, 3, 4)
    pt = transform * p
    assert pt == Point(5, 3, 4)


def test_sheer_x_z():
    transform = Shearing(0, 1, 0, 0, 0, 0)
    p = Point(2, 3, 4)
    pt = transform * p
    assert pt == Point(6, 3, 4)


def test_sheer_y_x():
    transform = Shearing(0, 0, 1, 0, 0, 0)
    p = Point(2, 3, 4)
    pt = transform * p
    assert pt == Point(2, 5, 4)


def test_sheer_y_z():
    transform = Shearing(0, 0, 0, 1, 0, 0)
    p = Point(2, 3, 4)
    pt = transform * p
    assert pt == Point(2, 7, 4)


def test_sheer_z_x():
    transform = Shearing(0, 0, 0, 0, 1, 0)
    p = Point(2, 3, 4)
    pt = transform * p
    assert pt == Point(2, 3, 6)


def test_sheer_z_y():
    transform = Shearing(0, 0, 0, 0, 0, 1)
    p = Point(2, 3, 4)
    pt = transform * p
    assert pt == Point(2, 3, 7)
