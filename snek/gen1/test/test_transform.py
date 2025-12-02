
from snek.gen1.lib.math.transformations import Translation

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