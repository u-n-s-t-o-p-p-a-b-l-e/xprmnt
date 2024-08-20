#include <stdio.h>
#include <math.h>

int main() {
	double number;

	printf("Enter a number: ");
	scanf("%lf", &number);

	if (number < 0) {
		printf("Error: Please enter a non-negative number");
		return 1;
	}

	double squareRoot = sqrt(number);

	printf("Square root of %.2lf is: %.2lf\n", number, squareRoot);

	return 0;
}
