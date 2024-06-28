#include <stdio.h>
#include <stdlib.h>

#define IN 1
#define OUT 0
/*
 * print a histogram of the lengths of words typed in up to 10
 *
 * 1. count lengths of words
 * 2. add counts to array
 * 3. plot
 *
 *
 *
*/

int *length_counter(void)
{
    int i, wc, bc, cc, lc, status, c;
    int len_arr[10];

    int *len_arr2 = malloc(sizeof(int[10]));

    if (!len_arr2)
        return NULL;

    for (i = 0; i < 10; ++i) {
        len_arr2[i] = 0;
        //printf("%d ", len_arr2[i]);
    }


    wc = bc = cc = lc = status = 0;

    for (i = 0; i < 10; ++i) {
         len_arr[i] = 0;
         //printf("%d ", len_arr[i]);
    }

    while ((c = getchar()) != EOF) {
        if (c == ' ' || c == '\n' || c == '\t') {
            if (status == IN) {
                ++len_arr2[lc];
                lc = 0;
                status = OUT;
            }
            ++bc;
        }
        else {
            if (status == OUT) {
                ++wc;
                status = IN;
            }
            else
                ++lc;
            ++cc;
        }
        //printf("test: %d", lc);
    }
    if (status == IN) {
        ++len_arr2[lc];
        lc = 0;
    }
    printf("\n------------------------------------\n");
    printf("wc: %d\tbc: %d\tcc: %d\n", wc, bc, cc);

    //for (i = 0; i < 10; ++i)
    //    printf("%d ", len_arr2[i]);

    return len_arr2;
}

void plotter(int *len_arr)
{
    //int i;
    //printf("printing from plotter!\n");
    //for (i = 0; i < 10; ++i)
    //     printf("%d ", len_arr[i]);
    //printf("\n");

    int i, i2;
    printf("\n   |\n");
    for (i = 9; i >= 0; --i) {
        if (i == 9)
            printf("%d |", i+1);
        else
            printf(" %d |", i+1);
        for (i2 = 0; i2 < len_arr[i]; ++i2)
            printf("000");
        printf("\n   |\n");
    }
    printf("   ----------------------------------------------------\n");
    printf("      1  2  3  4  5  6  7  8  9  10");
}






int main(void)
{
    printf("Type in a sentence and ");
    printf("I'll show you a histogram of the word lengths!\n");
    int *len_arr = length_counter();
    
    //int i;
    //printf("printing this in main!\n");
    //for (i = 0; i < 10; ++i)
    //    printf("%d ", len_arr[i]);
    //printf("\n");
    
    plotter(len_arr);
    free(len_arr);
    
    return 0;
}
