
from snek_ray.lib.sim.projectile import ProjectileSim
from snek_ray.lib.math.color import Color
from snek_ray.lib.rendering.canvas import Canvas

sim = ProjectileSim()
c = Canvas(900, 550)

while sim.projectile.position.y > 0:
    pos = sim.tick()

    if pos.x > 0 and pos.x < c.width and pos.y > 0 and pos.y < c.height:
        c.write_pixel(pos.x, pos.y, Color(255,255,255))

ppm_str = c.as_ppm()
# Save to repo root (note: we run via bazel)
path = "sim_result.ppm"
with open(path, "w", encoding="utf-8") as f:
  
    f.write(ppm_str)
