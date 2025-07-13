#include <gtest/gtest.h>
#include "tuple.h"
#include <cmath>
TEST(TupleTest, TupleCreation)
{
    // Create a tuple
    Tuple_t t = tuple(1.0, 2.0, 3.0, 0.0);

    // Check the values
    EXPECT_DOUBLE_EQ(t.x, 1.0);
    EXPECT_DOUBLE_EQ(t.y, 2.0);
    EXPECT_DOUBLE_EQ(t.z, 3.0);
    EXPECT_DOUBLE_EQ(t.w, 0.0);

    EXPECT_FALSE(is_point(t));
    EXPECT_TRUE(is_vector(t));
}

TEST(TupleTest, TuplePoint)
{
    Tuple_t p = point(1.0, 2.0, 3.0);
    EXPECT_TRUE(is_point(p));
    EXPECT_FALSE(is_vector(p));
    EXPECT_EQ(p.w, 1.0);
}

TEST(TupleTest, TupleVector)
{
    Tuple_t v = vector(1.0, 2.0, 3.0);
    EXPECT_TRUE(is_vector(v));
    EXPECT_FALSE(is_point(v));
    EXPECT_EQ(v.w, 0.0);
}

TEST(TupleTest, TupleSub)
{
    Tuple_t t_a = point(1.0, 2.0, -3.0);
    Tuple_t t_b = point(-1.0, 2.0, 3.0);

    Tuple_t t_diff = tuple_sub(t_a, t_b);
    Tuple_t t_expected = tuple(2.0, 0.0, -6.0, 0.0);

    EXPECT_TRUE(tuple_eq(t_diff, t_expected));
    EXPECT_TRUE(is_vector(t_diff));
}

TEST(TupleTest, TupleAdd)
{
    Tuple_t t_a = point(1.0, 2.0, -3.0);
    Tuple_t t_b = point(-1.0, 2.0, 3.0);

    Tuple_t t_sum = tuple_add(t_a, t_b);
    Tuple_t t_expected = tuple(0.0, 4.0, 0.0, 2.0);

    EXPECT_TRUE(tuple_eq(t_sum, t_expected));
    EXPECT_FALSE(is_vector(t_sum));
}

TEST(TupleTest, TupleNegate)
{
    Tuple_t t = point(1.0, -2.0, 3.0);
    Tuple_t t_negated = tuple_negate(t);
    Tuple_t t_expected = tuple(-1.0, 2.0, -3.0, -1.0);
    EXPECT_TRUE(tuple_eq(t_negated, t_expected));
}

TEST(TupleTest, TupleScalarMultiply)
{
    Tuple_t t = point(1.0, -2.0, 3.0);
    Tuple_t t_negated = tuple_scalar_mul(t, 2.0);
    Tuple_t t_expected = tuple(2.0, -4.0, 6.0, 2.0);
    EXPECT_TRUE(tuple_eq(t_negated, t_expected));
}

TEST(TupleTest, TupleScalarDivide)
{
    Tuple_t t = point(1.0, -2.0, 3.0);
    Tuple_t t_negated = tuple_scalar_div(t, 2.0);
    Tuple_t t_expected = tuple(0.5, -1.0, 1.5, 0.5);
    EXPECT_TRUE(tuple_eq(t_negated, t_expected));
}

TEST(TupleTest, TupleMagnitude)
{
    Tuple_t t1 = vector(0.0, 1.0, 0.0);
    double t1_m = tuple_mangintude(t1);
    double t1_expected = 1.0;
    EXPECT_EQ(t1_m, t1_expected);

    Tuple_t t2 = vector(-1.0, -2.0, -3.0);
    double t2_m = tuple_mangintude(t2);
    double t2_expected = sqrt(14.0);
    EXPECT_EQ(t2_m, t2_expected);
}

TEST(TupleTest, TupleNormalize)
{
    {
        Tuple_t t_raw = vector(4.0, 0.0, 0.0);
        Tuple_t t_normalize = tuple_normalize(t_raw);
        double t_mag = tuple_mangintude(t_normalize);
        double t_mag_expected = 1.0;
        Tuple_t t_norm_expected = tuple(1.0, 0.0, 0.0, 0.0);
        EXPECT_EQ(t_mag, t_mag_expected);
    }

    {
        Tuple_t t_raw = vector(1.0, 2.0, 3.0);
        Tuple_t t_normalize = tuple_normalize(t_raw);
        double t_mag = tuple_mangintude(t_normalize);
        double t_mag_expected = 1.0;
        EXPECT_EQ(t_mag, t_mag_expected);
    }
}

TEST(TupleTest, TupleDot)
{
    {
        Tuple_t t_a = vector(1.0, 2.0, 3.0);
        Tuple_t t_b = vector(2.0, 3.0, 4.0);

        double t_dot = tuple_dot(t_a,t_b);
        EXPECT_EQ(t_dot, 20);
    }


}

TEST(TupleTest, TupleCorss)
{
    {
        Tuple_t t_a = vector(1.0, 2.0, 3.0);
        Tuple_t t_b = vector(2.0, 3.0, 4.0);

        Tuple_t t_coss = tuple_cross(t_a,t_b);
        Tuple_t t_expected = vector(-1.0, 2.0, -1.0);
        EXPECT_TRUE(tuple_eq(t_coss, t_expected));
    }


}

