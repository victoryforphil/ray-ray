# Color Base
def float_eq(a, b, ep=0.001):
    return abs(a-b) < ep
class Color:
    def __init__(self, r,g,b):
        self.r = r
        self.g = g
        self.b = b

    def len(self):
        return 3
    def __eq__(self, value):
        return (float_eq(self[0],value[0])) and float_eq(self[1],value[1]) and float_eq(self[2],value[2])     
    def __getitem__(self, index):
        return (self.r, self.g, self.b)[index]
    def __add__(self, other):
        return Color(
            self.r + other.r, 
            self.g + other.g, 
            self.b + other.b
        )

    def __sub__(self, other):
        return Color(
            self.r - other.r, 
            self.g - other.g, 
            self.b - other.b, 
        )

    def __mul__(self, other):
        if isinstance(other, float):
            return Color(self.r * other, self.g * other, self.b * other)
        if isinstance(other, int):
            return Color(self.r * other, self.g * other, self.b * other)

        if isinstance(other, Color):
              return Color(
            self.r * other.r,
            self.g * other.g,
            self.b * other.b
        )
        return NotImplemented

    def __truediv__(self, other):
        if isinstance(other, float):
            return Color(self.r / other, self.g / other, self.b / other)
        if isinstance(other, int):
            return Color(self.r / other, self.g / other, self.b / other)
        return NotImplemented

    def __repr__(self):
        return f"Color({self.r} {self.g} {self.b})"
    
    def neg(self):
        return Color(-self.r, -self.g, -self.b)

    def magnitude(self):
        return (self.r**2 + self.g**2 + self.b**2) ** 0.5

    def normalize(self):
        mag = self.magnitude()
        return Color(self.r / mag, self.g / mag, self.b / mag)
    
    def hadamard_product(self, other):
        return Color(
            self.r * other.r,
            self.g * other.g,
            self.b * other.b
        )