def add(x, y):
    return x + y

def substract(x, y):
    return x - y

def multiply(x, y):
    return x * y

def divide(x, y):
    if y == 0:
        return "Error Division by Zero."
    else:
        return x / y

def main():
    print("Simple Calculator")
    print("Operations:")
    print("1. Add")
    print("2. Substract")
    print("3. Multiply")
    print("4. Divide")

    choice = input("Enter operation number (1/2/3/4): ")

    num1 = float(input("Enter first number: "))
    num2 = float(input("Enter second number: "))
