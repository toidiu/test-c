#include <mylib.h>
#include <stdio.h>

int main() {
    printf("\nHello, World! from main");

    print_hello();
    int sum = add_int(1, 2);
    printf("\nsum %d", sum);

    return 0;
}

