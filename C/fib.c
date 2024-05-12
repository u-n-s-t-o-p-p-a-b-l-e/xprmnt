
#include <stdio.h>

void fibonacci(int n) {
	int first = 0, second = 1, next, i;

	printf("Fibonacci sequence up to %d terms:\n", n);
	for (i = 0; i < n; i++) {
		if (i <= 1)
			next = i;
		else {
			next = first + second;
			first = second;
			second = next;
		}
		printf("%d ", next);
	}
	printf("\n");
}

int main() {
	int num_terms;

	printf("Enter the number of terms for Fibonacci sequence: ");
	scanf("%d", &num_terms);

	if (num_terms <= 0) {
		printf("Error: Plese enter a positive number of terms. \n");
		return 1;
	}

	fibonacci(num_terms);

	return 0;
}
