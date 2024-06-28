#include <stdio.h>

int main()
{
    float f, c;
    int lower, upper, step;

    lower = 0;
    upper = 300;
    step = 20;

    f = lower;
    printf("Fahrenheit to Celsius Table\n");
    printf(" F\t   C\n");
    printf("----------\n");
    while (f <= upper) {
        c = (5.0/9.0) * (f - 32.0);
        printf("%3.0f %6.1f\n", f, c);
        f = f + step;
    }
    printf("\n\nCelsius to Fahrenheit Table\n");
    printf(" C\t   F\n");
    printf("----------\n");
    c = upper;
    while (c >= lower) {
        f = c * (9.0 / 5.0) + 32.0;
        printf("%3.0f %6.1f\n", c, f);
        c = c - step;
    }
}
