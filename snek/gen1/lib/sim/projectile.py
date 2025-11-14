from snek.gen1.lib.math.tuple import Point, Vector

class Projectile:
    def __init__(self, pos, vel):
        self.position = pos
        self.velocity = vel

class Enviornment:
    def __init__(self, gravity, wind):
        self.gravity = gravity
        self.wind = wind

class ProjectileSim:
    def __init__(self):
        self.projectile = Projectile(Point(0, 1, 0), Vector(1, 1, 0).normalize())
        self.enviornment = Enviornment(Vector(0, -0.05, 0), Vector(-0.001, 0,0))
    
    def tick(self):
        self.projectile.position = self.projectile.position + self.projectile.velocity
        self.projectile.velocity = self.projectile.velocity + self.enviornment.gravity + self.enviornment.wind
        return  self.projectile.position