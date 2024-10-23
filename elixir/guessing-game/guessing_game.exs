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
