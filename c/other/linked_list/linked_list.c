#include <stdio.h>
#include <stdlib.h>

/*
struct Node {
    int data;
    struct Node* next;
};

int start(int d) {
    if (head != NULL) {
        printf("head not empty!\n");
        return -1;
    }
    struct Node new;

    new.data = d;
}

void add(int d) {
    if (head == NULL) {

    }
}
*/

struct Node {
    int data;
    struct Node* next;
};

struct Node* build_one_two_three(void) {
    struct Node* head = NULL;
    struct Node* second = NULL;
    struct Node* third = NULL;

    head = malloc(sizeof(int));
    second = malloc(sizeof(int));
    third = malloc(sizeof(int));

    head->data = 1;
    head->next = second;

    second->data = 2;
    second->next = third;

    third->data = 3;
    third->next = NULL;

    return head;
}

void print(struct Node* head) {
    if (head == NULL)
        printf("no list found!\n");
    else {
        printf("Linked list contains: [ ");
        struct Node* temp = head;
        while (temp != NULL) {
            printf("%d ", temp->data);
            temp = temp->next;
        }
        printf("]\n");
    }
}

void length(struct Node* head) {
    int count;
    struct Node* temp;
    count = 0;


}


int main(void) {
    struct Node* head;
    head = build_one_two_three();
    print(head);
}

