#include <stdio.h>

double add(double a, double b) { return a + b; }
double subtract(double a, double b) { return a - b; }
double multiply(double a, double b) { return a * b; }
double divide(double a, double b) { return b != 0 ? a / b : 0; }

int main() {
    double (*operations[])(double, double) =  {add, subtract, multiply, divide};
    char operators[] = {'+', '-', '*', '/'};
    int choice;
    double a, b;

    printf("Enter two numbers: ");
    scanf("%lf %lf", &a, &b);

    printf("Choose operation:\n");
    printf("1. Add\n2. Subtract\n3. Multiply\n4. Divide\n");
    scanf("%d", &choice);

    if (choice >= 1 && choice <= 4) {
        double result = operations[choice-1](a, b);
            printf("%.2f %c %.2f = %.2f\n", a, operators[choice-1], b, result);
        } else {
            printf("Invalid choice\n");
        }

        return 0;
}
