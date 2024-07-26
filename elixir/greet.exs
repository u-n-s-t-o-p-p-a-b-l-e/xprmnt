# greet.exs
defModule Greeter do
  def greet(name) do
    IO.puts("Hi, #{name}!")
  end
end

defmodule CLI do
  def main(args) do
    case args do
      [name] ->
        Greeter.greet(name)
      _ ->
        IO.puts("Usage: elixir greet.exs <name>")
      end
    end
end

CLI.main(System.argv())
