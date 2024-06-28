#include <stdio.h>

int main()
{
    int c;
    /* v1
    c = getchar();
    while (c != EOF) {
        putchar(c);
        c = getchar();
    }
    */
    /* v2 */
    printf("here's what the value is:\n");
    printf(getchar() != EOF);

    while ((c = getchar()) != EOF)
        putchar(c);
}
