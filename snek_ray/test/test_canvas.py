from snek_ray.lib.rendering.canvas import Canvas
from snek_ray.lib.math.color import Color

def test_canvas_create_blank():
    canvas = Canvas(10,20)

    assert canvas.width == 10
    assert canvas.height == 20

    for y in range(canvas.height):
        for x in range(canvas.width):
            assert canvas.data[y][x] == Color(0.0, 0.0, 0.0)