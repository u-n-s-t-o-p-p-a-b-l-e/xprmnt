defmodule GuessingGame do
  @min 1
  @max 100

  def main(_args) do
    IO.puts("Welcome to the guessing Game!")
    secret_number = :rand.uniform(@max)
    play(secret_number, @min, @max, 0)
  end

  
