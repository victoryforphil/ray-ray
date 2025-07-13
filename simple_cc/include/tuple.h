#ifndef RR_H_TUPLE

#define RR_H_TUPLE

#define RR_CONST_TUPLE_EPLISON 0.00001

typedef struct {
    double x;
    double y;
    double z;
    double w;
} Tuple_t;


/// Create a tuple with given x, y, z, w values
///
/// @param x The x coordinate
/// @param y The y coordinate
/// @param z The z coordinate
/// @param w The w coordinate (1.0 for points, 0.0 for vectors)
/// @return A Tuple_t instance initialized with the provided values
Tuple_t tuple(double x, double y, double z, double w);

/// Create a vector-style tuple (w = 0.0)
///
/// @param x The x coordinate
/// @param y The y coordinate
/// @param z The z coordinate
/// @return A Tuple_t instance initialized with the provided values
Tuple_t vector(double x, double y, double z);

/// Create a point-style tuple (w = 1.0)
///
/// @param x The x coordinate
/// @param y The y coordinate
/// @param z The z coordinate
/// @return A Tuple_t instance initialized with the provided values
Tuple_t point(double x, double y, double z);

/// Check if the tuple represents a point (w = 1.0)
/// @return true if the tuple is a point, false otherwise
bool is_point(const Tuple_t &t);

/// Check if the tuple represents a vector (w = 0.0)
/// @return true if the tuple is a vector, false otherwise
bool is_vector(const Tuple_t &t);


/// Compare tuples with-in range of RR_CONST_TUPLE_EPLISON
///
/// @param t_a Tuple #A to compare
/// @param t_b Tuple #B to compare
/// @return bool equalty
bool tuple_eq(const Tuple_t &t_a, const Tuple_t &t_b);

Tuple_t tuple_add(const Tuple_t &t_a, const Tuple_t &t_b);

Tuple_t tuple_sub( const Tuple_t &t_a, const Tuple_t &t_b);

Tuple_t tuple_negate(const Tuple_t &t);

Tuple_t tuple_scalar_mul(const Tuple_t &t, const double &scalar);

Tuple_t tuple_scalar_div(const Tuple_t &t, const double &scalar);

double tuple_mangintude(const Tuple_t &t);

Tuple_t tuple_normalize(const Tuple_t &t);

double tuple_dot(const Tuple_t &t_a, const Tuple_t &t_b);

Tuple_t tuple_cross(const Tuple_t &t_a, const Tuple_t &t_b);


void tuple_print(const Tuple_t &t);

#endif