// example1.c

#include <stdint.h>
#include <stdio.h>

extern int32_t suma(int32_t a, int32_t b);

int main (int argc, char **argv) {
    int a = 3, b = 2, s = 0;

    s = suma(a, b);
    printf("%d + %d = %d\n", a, b, s);
    return 0;
}
