#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define HEAP_SIZE 1024
#define MAX_ROOTS 100

typedef struct Object {
    size_t size;
    char marked;
} Object;

char heap[HEAP_SIZE];
Object* roots[MAX_ROOTS];
int root_count = 0;

void* gc_alloc(size_t size) {
    static char* free_ptr = heap;
    if (free_ptr + sizeof(Object) + size > heap + HEAP_SIZE) {
        printf("Out of memory\n");
        return NULL;
    }
    Object* obj = (Object*)free_ptr;
    obj->size = size;
    obj->marked = 0;
    free_ptr += sizeof(Object) + size;
    return obj + 1;
}

void add_root(void* ptr) {
    if (root_count < MAX_ROOTS) {
        roots[root_count++] = ptr - sizeof(Object);
    } else {
        printf("Too many roots\n");
    }
}

void mark(Object* obj) {
    if (obj->marked) return;
    obj->marked = 1;
    for (char* p = (char*)(obj + 1); p < (char*)(obj + 1) + obj->size; p += sizeof(void*)) {
        Object* child = *(Object**)p - 1;
        if (child >= (Object*)heap && child < (Object*)(heap + HEAP_SIZE)) {
            mark(child);
        }
    }
}

void sweep() {
    for (Object* obj = (Object*)heap; (char*)obj < heap + HEAP_SIZE; obj = (Object*)((char*)(obj + 1) + obj->size)) {
        if (!obj->marked) {
            printf("Freeing object at %p\n", obj + 1);
        } else {
            obj->marked = 0;
        }
    }
}

void gc() {
    for (int i = 0; i < root_count; i++) {
        mark(roots[i]);
    }
    sweep();
}

int main() {
    int* a = gc_alloc(sizeof(int));
    add_root(a);
    *a = 10;

    int* b = gc_alloc(sizeof(int));
    *b = 20;

    gc();

    return 0;
}
