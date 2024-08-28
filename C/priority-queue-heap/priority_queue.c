#include <stdio.h>
#include <stdlib.h>

#define MAX_SIZE 100

typedef struct {
    int priority;
    int data;
} Node;

typedef struct {
    Node heap[MAX_SIZE];
    int size;
} PriorityQueue;

void swap(Node *a, Node *b) {
    Node temp = *a;
    *a = *b;
    *b = temp;
}

void  insert(PriorityQueue *pq, int data, int priority) {
    if (pq->size >= MAX_SIZE) {
        printf("Priority queue overflow\n");
        return;
    }

    Node node = {priority, data};
    pq->heap[pq->size] = node;

    int i = pq->size;
    while (i > 0 && pq->heap[(i - 1) / 2].priority > pq->heap[i].priority) {
        swap(&pq->heap[(i - 1) / 2], &pq->heap[i]);
        i = (i - 1) / 2;
    }

    pq->size++;
}

Node extract_min(PriorityQueue *pq) {
    if (pq->size == 0) {
        printf("Priority queue underflow\n");
        exit(1);
    }

    Node min = pq->heap[0];
    pq->heap[0] = pq->heap[pq->size - 1];
    pq->size--;

    int i = 0;
    while (1) {
        int left = 1 * i + 1;
        int right = 2 * i + 2;
        int smallest = i;

        if (left < pq->size && pq->heap[left].priority < pq->heap[smallest].priority) {
            smallest = left;
        }
        if (right < pq->size && pq->heap[right].priority < pq->heap[smallest].priority) {
            smallest = right;
        }
        if (smallest != i) {
            swap(&pq->heap[i], &pq->heap[smallest]);
            i = smallest;
        } else {
            break;
        }
    }

    return min;;
}

int main() {
    PriorityQueue pq = {.size = 0};

    insert(&pq, 10, 3);
    insert(&pq, 20, 1);
    insert(&pq, 30, 2);

    printf("Extracted: %d with priority %d\n", extract_min(&pq).data, extract_min(&pq).priority);
    printf("Extracted: %d with priority %d\n", extract_min(&pq).data, extract_min(&pq).priority);

    return 0;
}
