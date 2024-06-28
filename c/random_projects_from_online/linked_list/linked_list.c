#include <stdio.h>
#include <stdlib.h>

struct node* head = NULL;

struct node {
    int data;
    struct node* next;
};

void create() {
    struct node* temp;
    temp = (struct node*)malloc(sizeof(struct node));
    printf("Enter the data of this node: ");
    scanf("%d", &temp->data);
    temp->next = NULL;
    if (head == NULL) {
        head = temp;
    }
    else {
        struct node* ptr = head;
        while (ptr->next != NULL ) {
            ptr = ptr->next;
        }
        ptr->next = temp;
    }
    printf("Created a node!\n");
}

void display(void) {
    if (head == NULL) {
        printf("Linked list is empty!");
        return;
    }
    printf("Linked list: ");
    struct node* ptr = head;
    while (ptr != NULL) {
        printf("%d ", ptr->data);
        ptr = ptr->next;
    }
    printf("\n");
}

void insert_first (void) {
    if (head == NULL) {
        printf("Linked list is empty!");
        return;
    }
    struct node* temp;
    temp = (struct node*)malloc(sizeof(struct node));
    printf("Enter the data of this node: ");
    scanf("%d", &temp->data);
    temp->next = NULL;
    if (head==NULL) {
        head = temp;
        return;
    }
    else {
        temp->next = head;
        head = temp;
    }
    printf("Inserted at first position!");
}

void insert_pos(void) {
    if (head == NULL) {
        printf("Linked list is empty!");
        return;
    }
    int i, pos;
    struct node* temp;
    struct node* iter;
    temp = (struct node*)malloc(sizeof(struct node*));
    iter = (struct node*)malloc(sizeof(struct node*));
    printf("Enter the data of this node: ");
    scanf("%d", &temp->data);
    printf("Enter the position of this node: ");
    scanf("%d", &pos);
    iter = head;

    /*
     * if there are three nodes:
     * pos == 0:
     *
     *
     *
     *
     *
     *
     */
    if (pos == 0 && head == NULL) {
        head = temp;
        return;
    }
    for (i = 0; head != NULL; ++i) {
        if (i == pos) {



        }
        head = head->next;
    }
    if (i == pos - 1) {

    }





    //for (i = 0; i <= pos + 1; ++i) {
    //    //iter = head->next;
    //    if (pos == 0) {
    //        head = temp->next;
    //        temp = head;
    //        printf("The data at index %d is %d", pos, temp->data);
    //        return;
    //    }
    //    else if (i == pos) {
    //        temp = head->next;
    //        iter = temp->next;
    //        printf("The data at index %d is %d", pos, temp->data);
    //    }
    //    


    //}







    //for (i = 0; iter->next != NULL; ++i) {
    //    iter = iter->next;
    //}
    //iter = head;
    //if (pos <= i) {
    //    for (i = 0; i < pos; ++i) {
    //        iter = iter->next;




    //    }
    //}



 
}

void delete_first(void) {
    if (head == NULL) {
        printf("Linked list is empty!");
        return;
    }
    struct node* ptr = head;
    head = head->next;
    free(ptr);
    printf("First node deleted\n");
}

void delete_end(void) {
    if (head == NULL) {
        printf("Linked list is empty!");
        return;
    }
    else if (head->next == NULL) {
        struct node* ptr = head;
        head = ptr->next;
    }
    else {
        struct node* ptr = head;
        struct node* prev_ptr = NULL;
        while (ptr->next != NULL) {
            prev_ptr = ptr;
            ptr = ptr->next;
        }
    prev_ptr->next = NULL;
    free(ptr);
    printf("Last node deleted!\n");
    }
}


int main(void) {
    create();
    create();
    display();
    delete_first();
    display();
    create();
    delete_end();
    display();
    insert_first();
    display();
}
