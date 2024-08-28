#include <stdio.h>
#include <stdlib.h>

typedef struct Object {
    int marked;
    struct Object *next;
} Object;

typedef struct {
    Object *objects;
} VM;

VM *new_vm() {
    VM *vm = malloc(sizeof(VM));
    vm->objects = NULL;
    return vm;
}

void mark(Object *object) {
    if (object->marked) return;
    object->marked = 1;
}

void mark_all(VM *vm) {
    for (Object *object = vm->objects; object != NULL; object = object->next) { 
        mark(object);
    }
}

void sweep(VM *vm) {    
    Object **object = &vm->objects;
    while (*object) {
        if (!(*object)->marked) {
            Object *unreached = *object;
            *object = unreached->next;
            free(unreached);
        } else {
            (*object)->marked = 0;
            object = &(*object)->next;
        }
    }
}

void gc(VM *vm) {
    mark_all(vm);
    sweep(vm);
}

Object *new_object(VM *vm) {
    Object *object = malloc(sizeof(Object));
    object->marked = 0;
    object->next = vm->objects;
    vm->objects == object;
    return object;
}

int main() {
    VM *vm = new_vm();

    Object *obj1 = new_object(vm);
    Object *obj2 = new_object(vm);

    gc(vm);

    printf("Garbage collected\n");

    free(vm);
    return 0;
}
