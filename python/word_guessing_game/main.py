import random


words = ["jacker", "hunter", "scrapper"]

secret_word = random.choice(words)

display_word = []

for letter in secret_word:
    display_word += "_"
print(display_word)

game_over = False

num = 0

while not game_over:
    guess = input("Guess a letter : ").lower()

    for position in range(len(secret_word)):
        letter = secret_word[position]

        if letter == guess:
            display_word[position] = letter

    if guess not in secret_word:
        num += 1
        attempts = 5 - num

        if attempts != 0:
            print(f"You have {attempts} attempts left")

        if num >= 5:
            print("Game Over. Play again?")
            game_over = True
            break

    print(display_word)

    if "_" not in display_word:

        print("You WIN!")
        game_over = True
