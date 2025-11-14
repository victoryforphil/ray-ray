# Tuple Base
# Note: w=1.0 is a point , w=0.0 is a vector

def Vector(x, y, z):    
    return Tuple(x, y, z, 0.0)

def Point(x, y, z):
    return Tuple(x, y, z, 1.0)

class Tuple:
    def __init__(self, x, y, z, w):
        self.x = x  
        self.y = y
        self.z = z
        self.w = w

    def len(self):
        return 4
    def __eq__(self, value):
        return (self[0] == value[0]) and  (self[1] == value[1])    and  (self[2] == value[2])   and (self[3] == value[3])     
    def __getitem__(self, index):
        return (self.x, self.y, self.z, self.w)[index]
    def __add__(self, other):
        return Tuple(
            self.x + other.x, 
            self.y + other.y, 
            self.z + other.z, 
        self.w + other.w)
    
    def __sub__(self, other):
        return Tuple(
            self.x - other.x, 
            self.y - other.y, 
            self.z - other.z, 
        self.w - other.w)
    
    def __mul__(self, other):
        if isinstance(other, float):
            return Tuple(self.x * other, self.y * other, self.z * other, self.w * other)
        if isinstance(other, int):
            return Tuple(self.x * other, self.y * other, self.z * other, self.w * other)
        return NotImplemented

    def __truediv__(self, other):
        if isinstance(other, float):
            return Tuple(self.x / other, self.y / other, self.z / other, self.w / other)
        if isinstance(other, int):
            return Tuple(self.x / other, self.y / other, self.z / other, self.w / other)
        return NotImplemented

    def __repr__(self):
        return f"Tuple({self.x} {self.y} {self.z} {self.w})"
    
    
    def is_point(self):
        return self.w == 1.0
    def is_vector(self):
        return self.w == 0.0
    
    def neg(self):
        return Tuple(-self.x, -self.y, -self.z, -self.w)
    
    def magnitude(self):    
        return (self.x**2 + self.y**2 + self.z**2 + self.w**2) ** 0.5
    
    def normalize(self):
        mag = self.magnitude()
        return Tuple(self.x / mag, self.y / mag, self.z / mag, self.w / mag)
    
    def dot(self, other):
        return (
            self.x * other.x +
            self.y * other.y + 
            self.z * other.z + 
            self.w * other.w 
        )
    
    def cross(self, other):
        return Vector(
            self.y * other.z - self.z * other.y, 
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
