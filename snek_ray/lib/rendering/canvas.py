from snek_ray.lib.math.color import Color

class Canvas:
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.data = [[Color(0.0,0.0,0.0) for _ in range(width)] for _ in range(height)]

    def write_pixel(self, x, y, color):
        self.data[int(y)][int(x)] = color

    def pixel_at(self, x, y):
        return self.data[int(y)][int(x)]

    def as_ppm(self):

        ppm_string = "P3\n" # Start with magic number p3

        # Append {width} {height}
        ppm_string += f"{self.width} {self.height}\n"

        # Add max color (255)
        ppm_string += "255\n"

        # Add each color row by row to PPM data
        for y in range(self.height):
            for x in range(self.width):
                pixel = self.pixel_at(x,y) 
                r = max(0, pixel.r)
                g = max(0, pixel.g)
                b = max(0, pixel.b)

            
                r = int(min(r * 255, 255))
                g = int(min(g * 255, 255))
                b = int(min(b * 255, 255))
                ppm_string += f"{r} {g} {b} "
            ppm_string += "\n" #End of image row 
        return ppm_string