#include <stdio.h>
#include <stdlib.h>

typedef enum { RED, BLACK } Color;

typedef struct Node {
  int data;
  Color color;
  struct Node *left, *right, *parent;
} Node;

Node *root = NULL;

void leftRotate(Node *x) {
  Node *y = x->right;
  x->right = y->left;
  if (y->left != NULL)
    y->left->parent = x;
  y->parent = x->parent;
  if (x->parent == NULL)
    root = y;
  else if (x == x->parent->left)
    x->parent->left = y;
  else
    x->parent->right = y;
  y->left = x;
  x->parent = y;
}

void rightRotate(Node *x) {
  Node *y = x->left;
  x->left = y->right;
  if (y->right != NULL)
    y->right->parent = x;
  y->parent = x->parent;
  if (x->parent == NULL)
    root = y;
  else if (x == x->parent->right)
    x->parent->right = y;
  else
    x->parent->left = y;
  y->right = x;
  x->parent = y;
}

void fixViolation(Node *z) {
  while (z->parent != NULL && z->parent->color == RED) {
    if (z->parent == z->parent->parent->left) {
      Node *y = z->parent->parent->right;
      if (y != NULL && y->color == RED) {
        z->parent->color = BLACK;
        y->color = BLACK;
        z->parent->parent->color = RED;
        z = z->parent->parent;
      } else {
        if (z == z->parent->right) {
          z = z->parent;
          leftRotate(z);
        }
        z->parent->color = BLACK;
        z->parent->parent->color = RED;
        rightRotate(z->parent->parent);
      }
    } else {
      Node *y = z->parent->parent->left;
      if (y != NULL && y->color == RED) {
        z->parent->color = BLACK;
        y->color = BLACK;
        z->parent->parent->color = RED;
        z = z->parent->parent;
      } else {
        if (z == z->parent->left) {
          z = z->parent;
          rightRotate(z);
        }
        z->parent->color = BLACK;
        z->parent->parent->color = RED;
        leftRotate(z->parent->parent);
      }
    }
  }
  root->color = BLACK;
}

void insert(int data) {
  Node *z = (Node *)malloc(sizeof(Node));
  z->data = data;
  z->left = z->right = z->parent = NULL;
  z->color = RED;

  Node *y = NULL;
  Node *x = root;

  while (x != NULL) {
    y = x;
    if (z->data < x->data)
      x = x->left;
    else
      x = x->right;
  }

  z->parent = y;
  if (y == NULL)
    root = z;
  else if (z->data < y->data)
    y->left = z;
  else
    y->right = z;

  fixViolation(z);
}

void inorder(Node *root) {
  if (root == NULL)
    return;
  inorder(root->left);
  printf("%d ", root->data);
  inorder(root->right);
}

int main() {
  insert(7);
  insert(3);
  insert(18);
  insert(10);
  insert(22);
  insert(8);
  insert(11);
  insert(16);

  printf("Inorder traversal: ");
  inorder(root);
  printf("\n");

  return 0;
}
