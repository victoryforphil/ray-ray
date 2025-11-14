#include <stdio.h>
#include <stdlib.h>
#include <memory.h>
#include "tuple.h"

typedef struct
{
    Tuple_t p_position;
    Tuple_t v_velocity;
} Projectile_t;

typedef struct
{
    Tuple_t v_gravity;
    Tuple_t v_wind;
} World_t;

Projectile_t projectile(Tuple_t p_position, Tuple_t v_velocity)
{
    return Projectile_t{
        .p_position = p_position,
        .v_velocity = v_velocity
    };
}

World_t world(Tuple_t v_wind, Tuple_t v_gravity)
{
    return World_t{
        .v_gravity = v_gravity,
        .v_wind = v_wind,
    };
}

Projectile_t tick(const World_t &world, const Projectile_t &projectile_t)
{
    Tuple_t pos = tuple_add(projectile_t.p_position, projectile_t.v_velocity);
    Tuple_t vec = tuple_add(projectile_t.v_velocity, tuple_add(world.v_gravity, world.v_wind));
    return projectile(pos, vec);
}

int main(int argc, char *argv[])
{

    Projectile_t p = projectile(vector(0.0, 1.0, 0.0), vector(1.0, 1.0, 0.0));

    World_t w = world(vector(0.0, -0.1, 0.0), vector(-0.01, 0.0, 0.0));

    while (p.p_position.y > 0)
    {
        p = tick(w, p);
        tuple_print(p.p_position);
    }

    return 0;
}