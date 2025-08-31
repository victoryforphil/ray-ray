#ifndef RR_H_COLOR

#define RR_H_COLOR

#define RR_CONST_COLLOR_EPLISON 0.001

typedef struct {
    double r;
    double g;
    double b;
} Color_t;


/// Create a color with given x, y, z, w values
///
/// @param r The r color value (0-1.0)
/// @param g The g color value (0-1.0)
/// @param b The b color value (0-1.0)
/// @return A Color_t instance initialized with the provided values
Color_t color(double r, double g, double b);

/// Compare colors with-in range of RR_CONST_COLOR_EPLISON
///
/// @param c_a Color #A to compare
/// @param c_b Color #B to compare
/// @return bool equalty
bool color_eq(const Color_t &c_a, const Color_t &c_b);

Color_t color_add(const Color_t &c_a, const Color_t &c_b);

Color_t color_sub( const Color_t &c_a, const Color_t &c_b);

Color_t color_scalar_mul(const Color_t &c, const double &scalar);

Color_t color_scalar_div(const Color_t &c, const double &scalar);

Color_t color_hadamard_product(const Color_t &c_a, const Color_t &c_b);
void color_print(const Color_t &c);

#endif