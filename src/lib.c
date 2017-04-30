#include <stdio.h>


void repeat(char *str, int n) {
    int i;

    for (i=0; i<n; i++) {
        printf("%s\n", str);
    }
}
