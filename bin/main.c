#include <mylib.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
    printf("\nHello, World! from main");

    print_hello();

    struct Sum *sum = (struct Sum *)malloc(sizeof(struct Sum));

    int ret = add_int(1, 2, sum);

    if (ret > 0) {
        return ret;
    }
    printf("\nsum %d", sum->value);

    return 0;
}

