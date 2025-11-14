from snek.gen1.lib.rendering.canvas import Canvas
from snek.gen1.lib.math.color import Color

def test_canvas_create_blank():
    canvas = Canvas(10,20)

    assert canvas.width == 10
    assert canvas.height == 20

    for y in range(canvas.height):
        for x in range(canvas.width):
            assert canvas.data[y][x] == Color(0.0, 0.0, 0.0)

def test_canvas_write_read():
    canvas = Canvas(10, 20)

    red = Color(1.0, 0., 0.)

    canvas.write_pixel(2,3, red)

    assert canvas.pixel_at(2,3) == Color(1., 0., 0.)

def test_canvas_ppm_header():
    canvas = Canvas(10, 20)

    ppm_str = canvas.as_ppm()

    lines = ppm_str.splitlines()

    assert lines[0] == "P3"
    assert lines[1] == "10 20"
    assert lines[2] == "255"

def test_canvas_ppm_color_data():
    canvas = Canvas(5, 3)

    c1 = Color(1.5, 0., 0.)
    c2 = Color(0., 0.5, 0.)
    c3 = Color(-0.5, 0., 1.)

    canvas.write_pixel(0,0, c1)
    canvas.write_pixel(2,1, c2)
    canvas.write_pixel(4,2, c3)

    ppm_str = canvas.as_ppm()

    lines = ppm_str.splitlines()
    
    assert lines[3] == "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 "
    assert lines[4] == "0 0 0 0 0 0 0 127 0 0 0 0 0 0 0 " 
    assert lines[5] == "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 " 