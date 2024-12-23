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

struct Node {
    TYPE val;
    struct Node* prev;
    struct Node* next;
};
// node
struct Node* create_node(TYPE val)
{
    struct Node* node = (struct Node*)malloc(sizeof(struct Node));
    node->val = val;
    node->prev = NULL;
    node->next = NULL;
    return node;
}

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
        printf("%d:%d", i, node->val);
        if (node->next) {
            printf(" -> ");
        }
        node = node->next;
        ++i;
    }
    printf("\n");
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
enum Bool prepend(struct DLL* dll, TYPE val)
{
    struct Node* newNode = create_node(val);

    dll->head->prev = newNode;
    newNode->next = dll->head;
    dll->head = newNode;

    ++dll->len;
    return TRUE;
}
enum Bool append(struct DLL* dll, TYPE val)
{
    struct Node* newNode = create_node(val);

    dll->tail->next = newNode;
    newNode->prev = dll->tail;
    dll->tail = newNode;

    ++dll->len;
    return TRUE;
}

enum Bool insert(struct DLL* dll, int pos, TYPE val)
{
    struct Node* newNode = create_node(val);

    dll->tail->next = newNode;
    newNode->prev = dll->tail;
    dll->tail = newNode;

    ++dll->len;
    return TRUE;
}

// main
int main()
{
    struct DLL* dll = create_dll(0);
    int len = dll->len;
    for (int i = 1; i <= 10; ++i) {
        if (append(dll, i) == FALSE) {
            printAndExit("!!! failed to append");
        }
        ++len;
    }
    for (int i = 1; i <= 8; ++i) {
        if (prepend(dll, i * -1) == FALSE) {
            printAndExit("!!! failed to prepend");
        }
        ++len;
    }
    print_dll(dll);

    assert(ith_val(dll, 1) == -7);
    assert(ith_val(dll, 9) == 1);
    assert(ith_val(dll, 15) == 7);

    assert(dll->len == len);

    free(dll);
    return 0;
}
