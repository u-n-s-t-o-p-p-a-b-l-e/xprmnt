#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define POOL_SIZE 1024
#define MAX_ALLOCS 100

typedef struct {
    char* start;
    size_t size;
    int used;
} Block;

typedef struct {
    char memory[POOL_SIZE];
    Block blocks[MAX_ALLOCS];
    int num_blocks;
} MemoryPool;

MemoryPool pool = {0};

void* pool_alloc(size_t size) {
    if (pool.num_blocks >= MAX_ALLOCS) return NULL;

    char* start = pool.memory;
    for (int i = 0; i < pool.num_blocks; i++) {
        if (!pool.blocks[i].used && pool.blocks[i].size >= size) {
            pool.blocks[i].used = 1;
            return pool.blocks[i].start;
        }
        start = pool.blocks[i].start + pool.blocks[i].size;
    }

    if (start + size > pool.memory + POOL_SIZE) return NULL;

    pool.blocks[pool.num_blocks].start = start;
    pool.blocks[pool.num_blocks].size = size;
    pool.blocks[pool.num_blocks].used = 1;
    pool.num_blocks++;

    return  start;
}

void pool_free(void* ptr) {
    for (int i = 0; i < pool.num_blocks; i++) {
        if (pool.blocks[i].start == ptr) {
            pool.blocks[i].used = 0;
            return;
        }
    }
}

int main() {
    char* str1 = pool_alloc(20);
    char* str2 = pool_alloc(30);

    strcpy(str1, "Hi");
    strcpy(str2, "There");

    printf("%s %s\n", str1, str2);

    pool_free(str1);
    pool_free(str2);

    return 0;
}
