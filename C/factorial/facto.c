#include <stdio.h>

unsigned long long factorial(int n) {
	if (n == 0 || n == 1)
		return 1;
	else
		return n * factorial(n -1 );
}

int main() {
	int num;

	printf("Enter a non-negative integer: \n");
	scanf("%d", &num);

	if (num < 0) {
		printf("Error : Please enter a non-neagative integer.\n");
		return 1;
	}

	unsigned long long result = factorial(num);

	printf("the factorial of %d is: %llu\n", num, result);

	return 0;
}
