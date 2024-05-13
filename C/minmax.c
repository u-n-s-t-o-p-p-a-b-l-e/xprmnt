#include <stdio.h>

int main() {
	int count, i, num, max, min;

	printf("Enter the count of numbers: ");
	scanf("%d", &count);

	if (count <= 0) {
		printf("Error: Please enter a positive count of numbers. \n");
		return 1;
	}

	printf("Enter %d numbers separated by spaces: \n", count);
	scanf("%d", &num);
	max = min = num;

	for (i = 1; i < count; i++) {
		scanf("%d", &num);
		if (num > max)
			max = num;
		if (num < min)
			min = num;
	}

	printf("Maximum number: %d\n", max);
	printf("Minimun number: %d\n", min);

	return 0;
}
