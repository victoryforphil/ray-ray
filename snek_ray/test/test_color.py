from snek_ray.lib.math.color import Color

def test_color_new():
    c = Color(-0.5, 0.4,1.7)

    assert c.r == -0.5
    assert c.g == 0.4
    assert c.b == 1.7


def test_color_add():
    c1 = Color(0.9, 0.6, 0.75)
    c2 = Color(0.7, 0.1, 0.25)

    ca = c1 + c2 
    assert ca == Color(1.6, 0.7, 1.0)

def test_color_sub():
    c1 = Color(0.9, 0.6, 0.75)
    c2 = Color(0.7, 0.1, 0.25)
    ca = c1 - c2 
    assert ca == Color(0.2, 0.5, 0.5)

def test_color_scalar_mul():
    c = Color(0.2, 0.5, 0.5)
    ca = c * 2
    assert(ca == Color(0.4, 1.0, 1.0))

def test_color_mul():
    c1 = Color(1.0, 0.2, 0.4)
    c2 = Color(0.9, 1.0, 0.1)
    ca = c1 * c2 
    assert ca == Color(0.9, 0.2, 0.04)