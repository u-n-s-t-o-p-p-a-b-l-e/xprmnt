#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MEMORY_SIZE 1024
#define MAX_BLOCKS 100

typedef struct {
    int id;
    int size;
    int start;
} MemoryBlock;

MemoryBlock memory[MAX_BLOCKS];
int block_count = 0;
char mem[MEMORY_SIZE];

void initialize_memory() {
    memset(mem, 0, MEMORY_SIZE);
}

int allocate(int size) {
    int start = 0;
    for (int i = 0; i < block_count; i++) {
        if (start + size <= memory[i].start) {
            memory[block_count].id = block_count;
            memory[block_count].size = size;
            memory[block_count].start = start;
            block_count++;
            memset(mem + start, 1, size);
            return block_count - 1;
        }
        start = memory[i].start + memory[i].size;
    }
    
    if (start + size <= MEMORY_SIZE) {
        memory[block_count].id = block_count;
        memory[block_count].size = size;
        memory[block_count].start = start;
        block_count++;
        memset(mem + start, 1, size);
        return block_count - 1;
    }
    return -1;
}

void deallocate(int id) {
    for (int i = 0; i < block_count; i++) {
    if (memory[i].id == id) {
        memset(mem + memory[i].start, 0, memory[i].size);
        memmove(&memory[i], &memory[i+1], (block_count - i - 1) * sizeof(MemoryBlock));
        block_count--;
        return;
        }
    }
}

void print_memory_map() {
    printf("Memory Map:\n");
    for (int i = 0; i < MEMORY_SIZE; i++) {
        if (i % 64 == 0) printf("\n");
        printf("%c", mem[i] ? '#' : '.');
    }
    printf("\n");
}

int main() {
    initialize_memory();
    int block1 = allocate(100);
    int block2 = allocate(200);
    int block3 = allocate(150);
    print_memory_map();
    deallocate(block2);
    int block4 = allocate(80);
    print_memory_map();
    return 0;

}
