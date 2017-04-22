// example2.c

#include <stdint.h>
#include <stdio.h>
#include <string.h>

struct Dato {
    int n;
    char *cadenac;
};

extern struct Dato* dato_crear(int, char*, int);
extern void dato_print(struct Dato*);
extern void dato_destruir(struct Dato*);

int main (int argc, char **argv) {
    int n = 5;
    char *cadena = "Esto es una cadena C";
    struct Dato *dato = dato_crear(n, cadena, strlen(cadena));

    printf("dato: %s\n", dato->cadenac);
    dato_print(dato);

    dato_destruir(dato);
    return 0;
}
