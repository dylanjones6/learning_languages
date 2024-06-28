#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdbool.h>

#define MAX_WORD_LEN 50
#define N_MISSES 3

char *get_word(void)
{
    int i;
    char c;
    //char word[MAX_WORD_LEN];
    char *word = malloc(sizeof(char[MAX_WORD_LEN + 1]));
    if (!word)
        printf("uh oh malloc() failed in get_word");

    printf("What do you want to be the special word?\n");
    scanf("%s", word);
    i = 0;
    while ((c = getchar()) != EOF) {
        if (c == '\n' || c == '\t' || c == ' ')
            break;
        else {
            word[i] = c;
            ++i;
        }
    }
    system("clear");
    //for (i = 0; i < MAX_WORD_LEN + 1; ++i)
    //    printf("%c ", word[i]);

    for (i = 0; i < MAX_WORD_LEN; ++i) {
        word[i] = tolower(word[i]);
    }

    //printf("The special word is: %s\n", word);
    
    return word;
}

char *remove_letter(char *dyn_letter_arr, char letter_guessed) 
{
    int i;

    for (i = 0; i < 26; ++i) {
        if (dyn_letter_arr[i] == letter_guessed) {
            dyn_letter_arr[i] = '_';
            return dyn_letter_arr;
        }
    }
    return NULL;
}

bool letter_checker(char *dyn_letter_arr, char letter_guessed)
{
    int i;

    for (i = 0; i < 26; ++i) {
        if (dyn_letter_arr[i] == letter_guessed) {
            return true;
        }
    }
    return false;
}

void print_letters(char *letter_arr, int letter_count, bool print_last)
{
    int i;
    if (print_last == true)
        for (i = 0; i < letter_count; ++i) {
            printf("%c ", letter_arr[i]);
        }
    else {
        for (i = 0; i < letter_count - 1; ++i)
            printf("%c ", letter_arr[i]);
    }
    printf("\n");
}

int *guess_checker(char *special_word, char letter_guessed, int letter_count)
{
    int i, nc;
    int *correct_ind = malloc(sizeof(letter_count));
    //int *correct_ind[letter_count];
    //correct_ind[0] = -1;
    nc = 0;

    for (i = 0; i < letter_count; ++i) {
        if (special_word[i] == letter_guessed) {
            ++nc;
            correct_ind[i] = 1;
        }
        else 
            correct_ind[i] = 0;
    }
    if (nc == 0)
        correct_ind[0] = -1;

    return correct_ind;
}

char *add_to_word_mod(char *word_mod, int *correct_ind,
                      char letter_guessed, int letter_count)
{
    for (int i = 0; i < letter_count; ++i) {
        if (correct_ind[i] == 1)
            word_mod[i] = letter_guessed;
    }
    return word_mod;
}

char get_letter(void)
{
    int i, i2, check, first_time;
    char letter;
    char letter_arr[] = {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
        'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'};

    check = 0;
    first_time = 1;

    printf("Guess a letter!\n");

    while (check != 1) {
        scanf("%c", &letter);
        letter = tolower(letter);
        for (i = 0; i < 26; ++i) {
            if (letter == letter_arr[i])
                check = 1;
        }
        if (check == 0 && first_time != 1)
            printf("That's not a letter of the alphabet! Try again.\n");
        first_time = 0;
    }

    printf("You picked: %c\n", letter);

    return letter;
}

int print_hangman(int misses)
{
    switch (misses) {

        case 0: 
            printf("\n");
            printf("----------------------------------------------\n");
            printf("|                                            |\n");
            printf("|       ---------------------                |\n");
            printf("|       |                   |                |\n");
            printf("|       |                   |                |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n"); 
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("----------------------------------------------\n");
            break;

        case 1:
            printf("\n");
            printf("----------------------------------------------\n");
            printf("|                                            |\n");
            printf("|       ---------------------                |\n");
            printf("|       |                   |                |\n");
            printf("|       |                   |                |\n");
            printf("|       |                .......             |\n");
            printf("|       |                .     .             |\n"); 
            printf("|       |                .     .             |\n");
            printf("|       |                .......             |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("----------------------------------------------\n");
            break;

        case 2:
            printf("\n");
            printf("----------------------------------------------\n");
            printf("|                                            |\n");
            printf("|       ---------------------                |\n");
            printf("|       |                   |                |\n");
            printf("|       |                   |                |\n");
            printf("|       |                .......             |\n");
            printf("|       |                .     .             |\n"); 
            printf("|       |                .     .             |\n");
            printf("|       |                .......             |\n");
            printf("|       |                \\  |  /             |\n");
            printf("|       |                 \\ | /              |\n");
            printf("|       |                  \\|/               |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("|       |                                    |\n");
            printf("----------------------------------------------\n");
            break;

        case 3:
            printf("\n");
            printf("----------------------------------------------\n");
            printf("|                                            |\n");
            printf("|       ---------------------                |\n");
            printf("|       |                   |                |\n");
            printf("|       |                   |                |\n");
            printf("|       |                .......             |\n");
            printf("|       |                .     .             |\n"); 
            printf("|       |                .     .             |\n");
            printf("|       |                .......             |\n");
            printf("|       |                \\  |  /             |\n");
            printf("|       |                 \\ | /              |\n");
            printf("|       |                  \\|/               |\n");
            printf("|       |                   |                |\n");
            printf("|       |                   |                |\n");
            printf("|       |                  /\\                |\n");
            printf("|       |                 /  \\               |\n");
            printf("|       |                /    \\              |\n");
            printf("|       |                                    |\n");
            printf("----------------------------------------------\n");
            break;
    }
    return 0;
}

    int main(void)
{
    char avail_letter_arr[] = {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
        'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'};
    int misses = 0;
    int i, i2, letter, letter_count;
    int letter_check = 0;
    char letter_guessed;
    //char *special_word[MAX_WORD_LEN + 1];
    
    //char *avail_letter_arr = malloc(sizeof(char[26]));
    char *dyn_letter_arr = malloc(sizeof(char[26]));
    //char *special_word = malloc(sizeof(char[MAX_WORD_LEN + 1]));
    if (!dyn_letter_arr)
        return -1;

    for (i = 0; i < 26 + 1; ++i)
        dyn_letter_arr[i] = avail_letter_arr[i];

    
    printf("Let's play hangman! First, someone else has to put in a word to guess!\n");
    printf("------------------------------------------------\n");
    //special_word = get_word(special_word);
    //char *special_word = get_word();
    char *special_word = get_word();
    letter_count = strlen(special_word);
    char *special_word_alloc = malloc(sizeof(char[letter_count]));
    for (i = 0; i < letter_count; ++i)
        special_word_alloc[i] = special_word[i];
   
    char *word_mod = malloc(sizeof(char[letter_count]));
    for (i = 0; i < letter_count; ++i) {
        word_mod[i] = '_';
    }
    //print_letters(word_mod);

    //letter_count = 0;
    //for (i = 0; i < MAX_WORD_LEN + 1; ++i) {
    //    for (i2 = 0; i2 < 26 + 1; ++i2) {
    //        if (special_word[i] == avail_letter_arr[i2])
    //            ++letter_count;
    //    }
    //}

    //printf("%lu", strlen(special_word));
    //printf("%s\n", special_word);
    //printf("%d\n", letter_count);
    //printf("%s\n", dyn_letter_arr);

    


    //printf("test\n");
    printf("Okay player! Time to begin!\n");

    int *correct_ind = malloc(sizeof(int[letter_count]));

    for (i = 0; i < letter_count; ++i)
        correct_ind[i] = 0;

    while (misses < N_MISSES) {
        print_hangman(misses); //bring this back!
        printf("\n");
        while (letter_check == 0) {
            print_letters(word_mod, strlen(word_mod), true);
            printf("\n");
            print_letters(dyn_letter_arr, strlen(dyn_letter_arr), false);
            printf("\n");
            letter_guessed = get_letter();
            if (letter_checker(dyn_letter_arr, letter_guessed))
                letter_check = 1;
            else
                printf("You already guessed %d!\n", letter_guessed);
                continue;
        }
        //int size = sizeof(avail_letter_arr);
        //printf("%s", dyn_letter_arr);

        //memcpy(avail_letter_arr, remove_letter(avail_letter_arr, letter_guessed), size);
        dyn_letter_arr = remove_letter(dyn_letter_arr, letter_guessed);

        correct_ind = guess_checker(special_word_alloc,
                                    letter_guessed, letter_count);

        if (correct_ind[0] != -1) {
            printf("Nice! You got it!\n");
            //add chosen letter to word_mod
            word_mod = add_to_word_mod(word_mod, correct_ind, letter_guessed, letter_count);
        }
        else {
            printf("Uh oh...\n");
            ++misses;
            if (misses == N_MISSES) {
                print_hangman(3);
                printf("Better luck next time!\n");
                break;
            }
        }
        //printf("%s", dyn_letter_arr);
        letter_check = 0;

        //misses = 6;
        i2 = 0;
        for (i = 0; i < letter_count; ++i) {
            if (word_mod[i] == special_word_alloc[i])
                ++i2;
        }
        if (i2 == letter_count) {
            printf("You win, good job guessing \"%s\"!\n", special_word_alloc);
            break;
        }

    }
    free(dyn_letter_arr);
    free(word_mod);
    free(special_word_alloc);
    free(correct_ind);

    return 0;
}
