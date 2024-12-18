#include "list.h"

#include <stdio.h>

#include "../api/mylib.h"

int print_hello() {
    printf("\nHello, World from mylib!");
    return 0;
}

int add_int(int a, int b, struct Sum *sum) {
    sum->value = a + b;
    return 0;
}

