#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_KEYS 3
#define MAX_CHILDREN (MAX_KEYS + 1)

typedef struct {
    int id;
    char name[50];
} Record;

typedef struct BTreeNode {
    int keys[MAX_KEYS];
    struct BTreeNode* children[MAX_CHILDREN];
    int num_keys;
    int is_leaf;
} BTreeNode;

BTreeNode* create_node() {
    BTreeNode* node = (BTreeNode*)malloc(sizeof(BTreeNode));
    node->num_keys = 0;
    node->is_leaf = 1;
    for (int i = 0; i < MAX_CHILDREN; i++)
        node->children[i] = NULL;
    return node;
}

void split_child(BTreeNode* parent, int index, BTreeNode* child) {
    BTreeNode* new_child = create_node();
    new_child->is_leaf = child->is_leaf;
    new_child->num_keys = MAX_KEYS / 2;

    for (int i = 0; i < MAX_KEYS / 2; i++)
        new_child->keys[i] = child->keys[i + MAX_KEYS / 2 + 1];

    if (!child->is_leaf) {
        for (int i = 0;  i <= MAX_KEYS / 2; i++)
            new_child->children[i] = child->children[i + MAX_KEYS / 2 + 1];
    }

    child->num_keys = MAX_KEYS / 2;

    for (int i = parent->num_keys; i > index; i--)
        parent->children[i + 1] = parent->children[i];

    parent->children[index + 1] = new_child;

    for (int i = parent->num_keys - 1; i >= index; i--)
        parent->keys[i + 1] = parent->keys[i];

    parent->keys[index] = child->keys[MAX_KEYS / 2];
    parent->num_keys++;
}

void insert_non_full(BTreeNode* node, int key) {
    int i = node->num_keys - 1;

    if (node->is_leaf) {
        while (i >=  0 && key < node->keys[i]) {
            node->keys[i + 1] = node->keys[i];
            i--;
        }
        node->keys[i + 1] = key;
        node->num_keys++;
        } else {
        while (i >= 0 && key < node->keys[i])
            i--;
        i++;
        if (node->children[i]->num_keys == MAX_KEYS) {
            split_child(node, i, node->children[i]);
            if (key > node->keys[i])
                i++;;
        }
        insert_non_full(node->children[i], key);
    }
}

void insert(BTreeNode** root, int key) {
    if ((*root)->num_keys == MAX_KEYS) {
        BTreeNode* new_root = create_node();
        new_root->is_leaf = 0;
        new_root->children[0] = *root;
        split_child(new_root, 0, *root);
        *root = new_root;
        insert_non_full(*root, key);
    } else {
        insert_non_full(*root, key);
    }
}

void print_tree(BTreeNode* node, int level) {
    if (node != NULL) {
        printf("Level %d: ", level);
        for (int i = 0; i < node->num_keys; i++)
            printf("%d ", node->keys[i]);
        printf("\n");
        if (!node->is_leaf) {
            for (int i = 0; i <= node->num_keys; i++)
                print_tree(node->children[i], level + 1);
        }
    }
}

int main() {
    BTreeNode* root = create_node();
    int keys[] = {3, 7, 1, 5, 2, 6, 4, 8};
    int n = sizeof(keys) / sizeof(keys[0]);

    for (int i = 0; i < n; i++)
        insert(&root, keys[i]);

    printf("B-Tree:\n");
    print_tree(root, 0);

    return 0;
}
