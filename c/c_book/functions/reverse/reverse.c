#include <stdio.h>
#include <stdlib.h>

#define STRLEN 1000
/*
 * Write a function reverse that reverses the character string s.
 * Use it to write a program that reverses its input a line at a time.
 *
 *
 *
*/

char *string_getter(void)
{
    char *string_arr = malloc(sizeof(char[STRLEN]));

    printf("Type in a string here: ");
    fgets(string_arr, sizeof(char[STRLEN]), stdin);
    printf("Here's the value: %s\n", string_arr);
    printf("%s", )
    
    return string_arr;
}

void reverse(char *s)
{
    int i;
    
    for (i = -1; i > -STRLEN; --i) {
        printf("%c", s[i]);
    }
}

int main(void)
{   
    char *string_arr = malloc(sizeof(char[STRLEN]));

    string_arr = string_getter();

    reverse(string_arr);

    return 0;
}



