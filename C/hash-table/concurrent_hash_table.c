#define _POSIX_C_SOURCE 200809L
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#define TABLE_SIZE 1000
#define NUM_THREADS 4

typedef struct Node {
    char* key;
    int value;
    struct Node* next;
} Node;

typedef struct {
    Node* buckets[TABLE_SIZE];
    pthread_rwlock_t locks[TABLE_SIZE];
} HashTable;

HashTable* create_table();
unsigned int hash(const char* key);
void insert(HashTable* table, const char* key, int value);
int get(HashTable* table, const char* key);
void* worker(void* arg);

HashTable* create_table() {
    HashTable* table = malloc(sizeof(HashTable));
    if (table == NULL) {
        perror("Failed to allocate memory for HashTable");
        exit(1);
    }
    for (int i = 0; i < TABLE_SIZE; i++) {
        table->buckets[i] = NULL;
        if (pthread_rwlock_init(&table->locks[i], NULL) != 0) {
            perror("Failed to inititalize read-write lock");
            exit(1);
        }
    }
    return table;
}

unsigned int hash(const char* key) {
    unsigned int hash = 0;
    while (*key) {
        hash = (hash * 31) + *key++;
    }
    return hash % TABLE_SIZE;
}

void insert(HashTable* table, const char* key, int value) {
    unsigned int index = hash(key);
    pthread_rwlock_wrlock(&table->locks[index]);

    Node* new_node = malloc(sizeof(Node));
    new_node->key = strdup(key);
    new_node->value = value;
    new_node->next = table->buckets[index];
    table->buckets[index] = new_node;

    pthread_rwlock_unlock(&table->locks[index]);
}

int get(HashTable* table, const char* key) {
    unsigned int index = hash(key);
    pthread_rwlock_rdlock(&table->locks[index]);

    Node* current = table->buckets[index];
    while (current) {
        if (strcmp(current->key, key) == 0) {
            int value = current->value;
            pthread_rwlock_unlock(&table->locks[index]);
            return value;
        }
        current = current->next;
    }

    pthread_rwlock_unlock(&table->locks[index]);
    return -1;
}

void* worker(void* arg) {
    HashTable* table = (HashTable*)arg;
    char key[20];
    for (int i = 0; i < 1000; i++) {
        sprintf(key, "key%d", i);
        insert(table, key, i);
    }
    for (int i = 0; i < 1000; i++) {
        sprintf(key, "key%d", i);
        int value = get(table, key);
        if (value != i) {
            printf("Error: Expected %d, got %d for key %s", i, value, key);
        }
    }
    return NULL;
}

int main() {
    HashTable* table = create_table();
    pthread_t threads[NUM_THREADS];

    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_create(&threads[i], NULL, worker, table);
    }

    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }

    printf("Concurrent operations completed successfully.\n");

    return 0;
}
