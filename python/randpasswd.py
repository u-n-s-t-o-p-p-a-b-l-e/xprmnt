import random
import string

def generate_password(length):
    characters = string.ascii + string.digits + string.punctuation
    password = ''.join(random.choice(characters) for _ in range(length))
    return password

def main():
    print("Random Password Generator")

    try:
        length = int(input("Enter the length of the password: "))
        if length <= 0:
            print("Length must be a positive integer.")
            return
    except ValueError:
        print("Invalid input. Please enter a valid integer.")
        return

    password = generate_password(length)
    print("Generated Password:", password)

if __name__ == "__main__":
    main()
        

