#include <stdio.h>
#include <stdlib.h>

#define ROWS 3
#define COLS 3

void print_matrix(int matrix[ROWS][COLS]) {
    for (int i = 0; i < ROWS; i++) {
        for (int j = 0; j < COLS; j++) {
            printf("%d ", matrix[i][j]);
        }
        printf("\n");
    }
}

void add_matrices(int a[ROWS][COLS], int b[ROWS][COLS], int result[ROWS][COLS])
{
    for (int i = 0; i < ROWS; i++) {
        for (int j = 0; j < COLS; j++) {
            result[i][j] = a[i][j] + b[i][j];
        }
    }
}

void multiply_matrices(int a[ROWS][COLS], int b[ROWS][COLS], int result[ROWS][COLS]) {
    for (int i = 0; i < ROWS; i++) {
        for (int j = 0; j < COLS; j++) {
            result[i][j] = 0;
            for (int k = 0; k < COLS; k++) {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
}

int main() {
    int matrix1[ROWS][COLS] = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
    int matrix2[ROWS][COLS] = {{9, 8, 7}, {6, 5, 4}, {3, 2, 1}};
    int result[ROWS][COLS];

    printf("Matrix 1:\n");
    print_matrix(matrix1);

    printf("\nMatrix 2:\n");
    print_matrix(matrix2);

    printf("\nMatrix Addition:\n");
    add_matrices(matrix1, matrix2, result);
    print_matrix(result);

    printf("\nMatrix Multiplication:\n");
    multiply_matrices(matrix1, matrix2, result);
    print_matrix(result);

    return 0;
}
