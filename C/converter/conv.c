#include <stdio.h>

int main() {
	double celsius, fahrenheit;

	printf("Enter temperature in Celcius: \n");
	scanf("%lf", &celsius);

	fahrenheit = (celsius * 9 / 5 ) + 32;

	printf("%.2lf Celsius is equal to %.2lf Fahrenheit. \n", celsius, fahrenheit);

	return 0;
}
