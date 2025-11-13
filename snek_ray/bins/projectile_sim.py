import matplotlib.pyplot as plt
from snek_ray.lib.sim.projectile import ProjectileSim

fig, ax = plt.subplots()
x_data = []
y_data = []

sim = ProjectileSim()

while sim.projectile.position.y > 0:
    pos = sim.tick()
    x_data.append(pos.x)
    y_data.append(pos.y)
    # Clear the previous plot

    
# Plot the data
ax.plot(x_data, y_data, 'bo-')  # 'bo-' means blue color, circle markers, solid line


plt.xlabel('X Position')
plt.ylabel('Y Position')
plt.title('Projectile Trajectory')
plt.grid(True)
plt.show()