#include <stdio.h>
#include <stdlib.h>

#define ARR_LEN 15

/*
 * 1. set L = 0 and R = n - 1
 * 2. if L > R, search unsuccessful
 * 3. set m (pos of middle element) to (L+M)/2
 * 4. if Am < T, set L to m + 1 and go to step 2
 * 5. if Am > T set R to m - 1 and go to step 2
 * 6. if Am = T, return m
 */

int bin_search(int T, int *sorted_arr) {
    int L, m, R, Am;
    L = 0;
    R = ARR_LEN - 1;

    while (L < R) {
        m = (L + R) / 2;
        Am = sorted_arr[m];
        if (Am < T) {
            L = m + 1;
           //printf("L: %d", L);
        }
        else if (Am > T)
            R = m - 1;
        else
            return m;
    }
    return -1;
}

int *bubble_sort(int *unsort_arr) {
    //int *sort_arr = malloc(sizeof(int[ARR_LEN]));
    int i, max, max_ind, R, temp;

    for (R = (ARR_LEN - 1); R > 0; --R) {
        max = unsort_arr[0];
        max_ind = 0;
        for (i = 0; i < (R + 1); ++i) {
            if (unsort_arr[i] > max) {
                max = unsort_arr[i];
                max_ind = i;
            }
        }
        temp = unsort_arr[R];
        unsort_arr[R] = max;
        unsort_arr[max_ind] = temp;
    }
    return unsort_arr;
}

void print_array(int *array) {
    for (int i = 0; i < ARR_LEN; ++i) {
        printf("%d ", array[i]);
    }
    printf("\n");
}

int main(void) {
    //int *unsort_arr = malloc(sizeof(int[ARR_LEN]));
    //int *pre_sort_arr = malloc(sizeof(int[ARR_LEN]));
    //int *sort_arr = malloc(sizeof(int[ARR_LEN]));
    int unsort_arr[] = {4, 6, 3, 2, 5, 0, 1, 8, 9, 7, 4, 7, 8, 0, 13};
    //int pre_sort_arr[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 10, 11, 12, 13};

    //printf("%d", target_ind);
    print_array(unsort_arr);
    int *sort_arr = bubble_sort(unsort_arr);
    print_array(sort_arr);
    int target_ind = bin_search(8, sort_arr);
    printf("%d\n", target_ind);
    //print_array(sort_arr);

    //free(sort_arr);
    //free(unsort_arr);

    return 0;
}
