#include <stdio.h>

// Tell C that this function exists in an external library
int rust_add(int a, int b);

int main() {
    int result = rust_add(2, 3);
    printf("Result: %d\n", result);
    return 0;
}