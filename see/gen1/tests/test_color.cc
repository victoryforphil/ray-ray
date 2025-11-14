#include <gtest/gtest.h>
#include "color.h"
#include <cmath>

TEST(ColorTest, ColorCreation){

    // Create a color
    Color_t c = color(-0.5, 0.4, 1.0);

    // Check the values
    EXPECT_DOUBLE_EQ(c.r, -0.5);
    EXPECT_DOUBLE_EQ(c.g, 0.4);
    EXPECT_DOUBLE_EQ(c.b, 1.0);
}


TEST(ColorTest, ColorSub)
{
    Color_t c_a = color(0.7, 0.2, -0.1);
    Color_t c_b = color(1.0, 0.2, 1.1);

    Color_t c_diff = color_sub(c_a, c_b);
    Color_t c_expected = color(-0.3,0.0,1.0);

    EXPECT_TRUE(color_eq(c_diff, c_expected));
}

TEST(ColorTest, ColorAdd)
{
    Color_t t_a = color(1.0, 2.0, -3.0);
    Color_t t_b = color(-1.0, 2.0, 3.0);

    Color_t t_sum = color_add(t_a, t_b);
    Color_t t_expected = color(0.0, 4.0, 0.0);

    EXPECT_TRUE(color_eq(t_sum, t_expected));
}

TEST(ColorTest, ColorScalarMultiply)
{
    Color_t c = color(0.2, 1.0, 0.5);
    double s = 2.0;
    Color_t c_scaled = color_scalar_mul(c, s);
    Color_t c_expected = color(0.4, 2.0, 1.0);
    EXPECT_TRUE(color_eq(c_scaled, c_expected));
}

TEST(ColorTest, ColorScalarDivide)
{
    Color_t c = color(0.2, 1.0, 0.5);
    double s = 2.0;
    Color_t c_scaled = color_scalar_div(c, s);
    Color_t c_expected = color(0.1, 0.5, 0.25);
    EXPECT_TRUE(color_eq(c_scaled, c_expected));
}

TEST(ColorTest, ColorHadamardProduct)
{
    Color_t c_a = color(0.2, 1.0, 0.5);
    Color_t c_b = color(1.0, -0.1, 0.5);
    Color_t c_scaled = color_hadamard_product(c_a, c_b);
    Color_t c_expected = color(0.2, -0.1, 0.25);
    EXPECT_TRUE(color_eq(c_scaled, c_expected));
}