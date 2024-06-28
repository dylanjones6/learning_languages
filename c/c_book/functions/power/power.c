#include <stdio.h>

int power(int m,int n)
{
    int out, c;
    
    if (n == 0)
        return 1;
    else if (n == 1)
        return m;
    else {
        for (c = 2; c <= n; ++c) {
            out = m * m;
        }
        return out;
    }
}



int main(void)
{
    int result, in, m, n;
    
    printf("Input an integer to be raised to a given power!");
    while ((in = getchar()) != EOF) //|| (in = getchar()) != '\n')
        m = in;


    printf("\nm is set to: %d \n", m);
    result = power(3, 1);
    //printf("%d\n", result);
}
