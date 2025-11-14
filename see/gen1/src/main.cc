#include <stdio.h>
#include <stdlib.h>

#include "tuple.h"

int main(int argc, char *argv[]) {


    printf("Hello, World!\n");

    Tuple_t t{
        .x = 1.0,
        .y = 2.0,
        .z = 3.0,
        .w = 4.0
    };

    printf("Tuple values: x=%.1f, y=%.1f, z=%.1f, w=%.1f\n", t.x, t.y, t.z, t.w);

    return 0;
}