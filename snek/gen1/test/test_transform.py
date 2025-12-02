
from snek.gen1.lib.math.transformations import Reflect, Translation, Scale

from snek.gen1.lib.math.tuple import Vector, Point


def test_transform_translation_matrix_mul():
    transform = Translation(Vector(5., -3., 2.))
    p = Point(-3., 4., 5.)

    transformed_p = transform * p
    expected_p = Point(2., 1., 7.)
    assert transformed_p.as_tuple() == expected_p


def test_transform_inverse_translate_mul():
    transform = Translation(Vector(5., -3., 2.))
    inv = transform.inverse()
    p = Point(-3., 4., 5.)

    transformed_p = inv * p
    expected_p = Point(-8.,7., 3.)
    assert transformed_p.as_tuple() == expected_p


def test_tranform_translate_unchanged_vectors():
    transform = Translation(Vector(5., -3., 2.))
    v = Vector(-3., 4., 5.)
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
