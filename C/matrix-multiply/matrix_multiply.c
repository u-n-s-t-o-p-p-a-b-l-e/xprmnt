#include <stdio.h>
#include <stdlib.h>
#include <omp.h>
#include <time.h>

#define N 1000

void initialize_matrix(double **matrix) {
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            matrix[i][j] = rand() / (double)RAND_MAX;
        }
    }
}

void multiply_matrices(double **A, double **B, double **C) {
    #pragma omp parallel for collapse(2)
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            C[i][j] = 0.0;
            for (int k = 0; k < N; k++) {
                C[i][j] +=  A[i][k] * B[k][j];
            }
        }
    }
}

int main() {
    double **A, **B, **C;
    double start_time, end_time;

    A = (double **)malloc(N * sizeof(double *));
    B = (double **)malloc(N * sizeof(double *));
    C = (double **)malloc(N * sizeof(double *));
    for (int i = 0; i < N; i++) {
        A[i] = (double *)malloc(N * sizeof(double));
        B[i] = (double *)malloc(N * sizeof(double));
        C[i] = (double *)malloc(N * sizeof(double));
    }

    srand(time(NULL));
    initialize_matrix(A);
    initialize_matrix(B);

    start_time = omp_get_wtime();
    multiply_matrices(A, B, C);
    end_time = omp_get_wtime();

    printf("Time taken: %f seconds\n", end_time - start_time);

    for (int i = 0;  i < N; i++) {
        free(A[i]);
        free(B[i]);
        free(C[i]);
    }
    free(A);
    free(B);
    free(C);

    return 0;

}
