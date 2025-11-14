#include "color.h"
#include <math.h>
#include <stdio.h>

Color_t color(double r, double g, double b)
{
    Color_t color;
    color.r = r;
    color.g = g;
    color.b = b;
    return color;
}
bool color_eq(const Color_t &c_a, const Color_t &c_b){
    Color_t t_diff = color_sub(c_a, c_b);

    return abs(t_diff.r < RR_CONST_COLLOR_EPLISON) && 
    abs(t_diff.g < RR_CONST_COLLOR_EPLISON) && 
    abs(t_diff.b < RR_CONST_COLLOR_EPLISON);
}

Color_t color_add(const Color_t &c_a, const Color_t &c_b){
    return Color_t{
        .r = c_a.r + c_b.r,
        .g = c_a.g + c_b.g,
        .b = c_a.b + c_b.b,
    };
}

Color_t color_sub( const Color_t &c_a, const Color_t &c_b){
    return Color_t{
        .r = c_a.r - c_b.r,
        .g = c_a.g - c_b.g,
        .b = c_a.b - c_b.b,
    };
}

Color_t color_scalar_mul(const Color_t &c, const double &scalar){
    return Color_t{
        .r = c.r * scalar,
        .g = c.g * scalar,
        .b = c.b * scalar
    };
}

Color_t color_scalar_div(const Color_t &c, const double &scalar){
    return Color_t{
        .r = c.r / scalar,
        .g = c.g / scalar,
        .b = c.b / scalar
    };
}

Color_t color_hadamard_product(const Color_t &c_a, const Color_t &c_b){
    return Color_t{
        .r = c_a.r * c_b.r, 
        .g = c_a.g * c_b.g, 
        .b = c_a.b * c_b.b, 
    };
}
void color_print(const Color_t &c);
