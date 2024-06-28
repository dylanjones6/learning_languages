#include <stdio.h>

/*
int x = 1, y = 2, z[10];
int *ip;        // ip is a pointer to int
ip = &x;
y = *ip;
*/

int main(void) {
    printf("Hello, world!\n");
    int x;
    x = 1;
    printf("value at x: %d\n", x);
    int *p;
    p = &x;
    printf("value of p: %p\n", p);
    printf("value pointed to by p: %d\n", *p);
    printf("incrememnt *p by 1...\n");
    (*p)++;
    printf("value of x: %d\n", x);
    printf("value of *p: %d\n", *p);
}
