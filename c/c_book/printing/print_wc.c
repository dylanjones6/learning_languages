#include <stdio.h>

#define IN 1 //means we're in a word
#define OUT 0 //means we're out of a word

//" this is a test input "



int main(void)
{
    int c, wc, status;
    
    wc = 0;
    status = OUT;
    while ((c = getchar()) != EOF) {
        if (c == '\n' || c == '\t' || c == ' ') {
            if (status == IN) {
                status = OUT;
            }
        }
        else {
            if (status == OUT) {
                status = IN;
                ++wc;
            }
        }
    }
    printf("%d\n", wc);



}
