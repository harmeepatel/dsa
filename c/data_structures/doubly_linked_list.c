#include <assert.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

typedef int TYPE;

enum Bool {
    FALSE,
    TRUE,
};

void printAndExit(char* str)
{
    printf("%s\n", str);
    exit(1);
}

// node
struct Node {
    TYPE val;
    struct Node* prev;
    struct Node* next;
};

struct Node* create_node(TYPE val)
{
    struct Node* node = (struct Node*)malloc(sizeof(struct Node));
    node->val = val;
    node->prev = NULL;
    node->next = NULL;
    return node;
}

// doubly linked list
struct DLL {
    unsigned int len;
    struct Node* head;
    struct Node* tail;
};

struct DLL* create_dll(TYPE val)
{
    struct DLL* dll = (struct DLL*)malloc(sizeof(struct DLL));
    struct Node* node = create_node(val);
    dll->len = 1;
    dll->head = node;
    dll->tail = node;
    return dll;
}

void print_dll(struct DLL* dll)
{
    int i = 0;
    struct Node* node = dll->head;

    while (node) {
        printf("%0*d:%2d", 2, i, node->val);
        if (node->next) {
            printf(" -> ");
        }
        node = node->next;
        ++i;
    }
    printf("\n");
}

void free_dll(struct DLL* dll)
{
    if (dll == NULL)
        return;

    struct Node* curr = dll->head;
    struct Node* next;

    while (curr) {
        next = curr->next;
        free(curr);
        curr = next;
    }

    free(dll);
}

// traversing
struct Node* ith_node(struct DLL* dll, int index)
{
    if (index < 0) {
        printAndExit("invalid index 0..");
    }
    int i = 0;
    struct Node* curr = dll->head;
    while (curr && i != index) {
        curr = curr->next;
        ++i;
    }
    if (curr == NULL) {
        printAndExit("index out of range!");
    }

    return curr;
}

TYPE ith_val(struct DLL* dll, int index)
{
    if (index < 0) {
        printAndExit("invalid index 0..");
    }
    int i = 0;
    struct Node* curr = dll->head;
    while (curr && i != index) {
        curr = curr->next;
        ++i;
    }
    if (curr == NULL) {
        printAndExit("index out of range!");
    }

    return curr->val;
}

struct Node* node_with_val(struct DLL* dll, TYPE needle)
{
    struct Node* curr = dll->head;
    while (curr && curr->val != needle) {
        curr = curr->next;
    }
    return curr;
}

// modify
enum Bool push_first(struct DLL* dll, TYPE val)
{
    struct Node* newNode = create_node(val);

    dll->head->prev = newNode;
    newNode->next = dll->head;
    dll->head = newNode;

    ++dll->len;
    return TRUE;
}

enum Bool push_last(struct DLL* dll, TYPE val)
{
    struct Node* newNode = create_node(val);

    dll->tail->next = newNode;
    newNode->prev = dll->tail;
    dll->tail = newNode;

    ++dll->len;
    return TRUE;
}

enum Bool push_at(struct DLL* dll, int pos, TYPE val)
{
    if (pos < 0 || pos > dll->len) {
        printAndExit("!!! cannot insert at invalid position");
    }
    if (pos == 0) {
        push_first(dll, val);
        return TRUE;
    }
    if (pos == dll->len) {
        push_last(dll, val);
        return TRUE;
    }
    struct Node* newNode = create_node(val);

    struct Node* ith = ith_node(dll, pos);

    newNode->prev = ith->prev;
    newNode->next = ith;
    ith->prev->next = newNode;
    ith->prev = newNode;

    ++dll->len;
    return TRUE;
}

TYPE pop_first(struct DLL* dll)
{
    TYPE val = dll->head->val;
    struct Node* head = dll->head;

    if (dll->len == 1) {
        printAndExit("!!! cannot delete the only value in linked list.");
    }
    dll->head = head->next;
    dll->head->prev = NULL;
    head->next = NULL;

    --dll->len;
    free(head);
    return val;
}
TYPE pop_last(struct DLL* dll)
{
    TYPE val = dll->tail->val;
    struct Node* tail = dll->tail;

    if (dll->len == 1) {
        printAndExit("!!! cannot delete the only value in linked list.");
    }
    dll->tail = tail->prev;
    dll->tail->next = NULL;
    tail->prev = NULL;

    --dll->len;
    free(tail);
    return val;
}
TYPE pop_at(struct DLL* dll, int pos)
{
    struct Node* ith = ith_node(dll, pos);
    TYPE val = ith->val;

    ith->prev->next = ith->next;
    ith->next->prev = ith->prev;

    ith->next = NULL;
    ith->prev = NULL;

    --dll->len;
    free(ith);
    return val;
}

// main
int main()
{
    struct DLL* dll = create_dll(0);
    int len = dll->len;

    // push_last
    for (int i = 1; i <= 8; ++i) {
        if (push_last(dll, i) == FALSE) {
            printAndExit("!!! failed to append");
        }
        ++len;
    }
    // push_first
    for (int i = 1; i <= 8; ++i) {
        if (push_first(dll, i * -1) == FALSE) {
            printAndExit("!!! failed to prepend");
        }
        ++len;
    }
    print_dll(dll);

    assert(ith_val(dll, 1) == -7);
    assert(ith_val(dll, 9) == 1);
    assert(ith_val(dll, 15) == 7);

    // push_at
    const int i_val = 99;
    static const int pos[] = { 0, 10, 10, 20 };
    for (int i = 0; i != sizeof(pos) / sizeof(pos[0]); ++i) {
        push_at(dll, pos[i], i_val);
        ++len;
    }
    print_dll(dll);

    // pop_first
    pop_first(dll);
    --len;
    // pop_last
    pop_last(dll);
    --len;
    print_dll(dll);

    // pop_at
    pop_at(dll, 10);
    --len;
    pop_at(dll, 9);
    --len;
    print_dll(dll);

    assert(dll->len == len);

    // replace
    struct Node* i = ith_node(dll, 8);
    i->val = 99;
    print_dll(dll);

    free_dll(dll);
    return 0;
}
