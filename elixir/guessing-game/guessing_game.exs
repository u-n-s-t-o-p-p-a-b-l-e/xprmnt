defmodule GuessingGame do
  @min 1
  @max 100

  def main(_args) do
    IO.puts("Welcome to the guessing Game!")
    secret_number = :rand.uniform(@max)
    play(secret_number, @min, @max, 0)
  end

  defp play(secret_number, min, max, attempts) do
    IO.puts("\nGuess a number between #{min} and #{max}:")
    guess = get_guess()

    cond do 
      guess == secret_number ->
        IO.puts("Congratulations! You've guessed the correct number: #{secret_number} in #{attempts + 1} attempts.")

      guess < secret_number ->
        IO.puts("Too low!")
        play(secret_number, min, guess + 1, max, attempts + 1)

      guess > secret_number ->
        IO.puts("Too high!")
        play(secret_number, min, guess -1, attempts + 1)
    end
  end

  defp get_guess do
    case IO.gets("> ") |> String.trim() |> Integer.parse() do
      {guess, _} -> guess
      :error ->
        IO.puts("Invalid input, please enter a number.")
        get_guess()
    end
  end
end

GuessingGame.main(System.argv())
