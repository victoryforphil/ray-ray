from snek_ray.lib.math.color import Color

class Canvas:
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.data = [[Color(0.0,0.0,0.0) for _ in range(width)] for _ in range(height)]

    def write_pixel(self, x, y, color):
        self.data[y][x] = color

    def pixel_at(self, x, y):
        return self.data[y][x]