#include "tuple.h"
#include <math.h>
#include <stdio.h>
Tuple_t tuple(double x, double y, double z, double w)
{
    Tuple_t t;
    t.x = x;
    t.y = y;
    t.z = z;
    t.w = w;
    return t;
}

Tuple_t point(double x, double y, double z)
{
    return tuple(x, y, z, 1.0);
}

Tuple_t vector(double x, double y, double z)
{
    return tuple(x, y, z, 0.0);
}

Tuple_t tuple_add(const Tuple_t &t_a, const Tuple_t &t_b)
{
    return Tuple_t{
        .x = t_a.x + t_b.x,
        .y = t_a.y + t_b.y,
        .z = t_a.z + t_b.z,
        .w = t_a.w + t_b.w,
    };
}

Tuple_t tuple_sub(const Tuple_t &t_a, const Tuple_t &t_b)
{
    return Tuple_t{
        .x = t_a.x - t_b.x,
        .y = t_a.y - t_b.y,
        .z = t_a.z - t_b.z,
        .w = t_a.w - t_b.w,
    };
}

// NOTE: Memory allocation to compare tuple (via tuple_sub) -
// 10/10 pro-programmer move.
// NOTE: Yes ill fix this.
bool tuple_eq(const Tuple_t &t_a, const Tuple_t &t_b)
{
    Tuple_t t_diff = tuple_sub(t_a, t_b);

    return abs(t_diff.x < RR_CONST_TUPLE_EPLISON) && abs(t_diff.y < RR_CONST_TUPLE_EPLISON) && abs(t_diff.z < RR_CONST_TUPLE_EPLISON) && abs(t_diff.w < RR_CONST_TUPLE_EPLISON);
}

Tuple_t tuple_negate(const Tuple_t &t)
{
    return Tuple_t{
        .x = -t.x,
        .y = -t.y,
        .z = -t.z,
        .w = -t.w,
    };
}

Tuple_t tuple_scalar_mul(const Tuple_t &t, const double &scalar)
{
    return Tuple_t{
        .x = t.x * scalar,
        .y = t.y * scalar,
        .z = t.z * scalar,
        .w = t.w * scalar,
    };
}

Tuple_t tuple_scalar_div(const Tuple_t &t, const double &scalar)
{
    return Tuple_t{
        .x = t.x / scalar,
        .y = t.y / scalar,
        .z = t.z / scalar,
        .w = t.w / scalar,
    };
}

double tuple_mangintude(const Tuple_t &t)
{
    return sqrt(
        pow(t.x, 2.) +
        pow(t.y, 2.) +
        pow(t.z, 2.) +
        pow(t.w, 2.));
}

Tuple_t tuple_normalize(const Tuple_t &t)
{
    double t_mag = tuple_mangintude(t);
    return tuple_scalar_div(t, t_mag);
}

double tuple_dot(const Tuple_t &t_a, const Tuple_t &t_b)
{
    return (t_a.x * t_b.x) +
           (t_a.y * t_b.y) +
           (t_a.z * t_b.z) +
           (t_a.w * t_b.w);
}


Tuple_t tuple_cross(const Tuple_t &t_a, const Tuple_t &t_b){
    return vector(
        (t_a.y * t_b.z) - (t_a.z * t_b.y),
        (t_a.z * t_b.x) - (t_a.x * t_b.z),
        (t_a.x * t_b.y) - (t_a.y * t_b.x));
}


bool is_point(const Tuple_t &t)
{
    return t.w == 1.0;
}

bool is_vector(const Tuple_t &t)
{
    return t.w == 0.0;
}

void tuple_print(const Tuple_t &t)
{
    printf("Tuple: (x=%.2f, y=%.2f, z=%.2f, w=%.2f)\n", t.x, t.y, t.z, t.w);
}