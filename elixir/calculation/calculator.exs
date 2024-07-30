defmodule Calculator do
  def main(args) do
    case args do
      [operation, operand1, operand2] ->
        operand1 = parse_number(operand1)
        operand2 = parse_number(operand2)
        perform_operation(operation, operand1, operand2)
      _ ->
        IO.puts("""
          Usage:
          elixir calculator.exs <operation> <operand1> <operand2>

          Operations:
          add       - Add two numbers
          substract - Subtract second number from first
          Multiply  - Multiply two numbers
          divide    - Divide first number by second
          """)
    end
  end

  defp parse_number(str) do
    case Integer.parse(str) do
      {int, ""} -> int
      _ -> String.to_float(str)
    end
  end

  defp perform_operation("add", operand1, operand2) do
    result = operand1 + operand2
    IO.puts("Result: #{result}")
  end

  defp perform_operation("subtract", operand1, operand2) do
    result = operand1 - operand2
    IO.puts("Result: #{result}")
  end

  defp perform_operation("multiply", operand1, operand2) do
    result = operand1 * operand2
    IO.puts("Result:  #{result}")
  end

  defp perform_operation("divide", operand1, operand2) do
    if operand2 == 0 do
      IO.puts("Error: Cannot divide by zero")
    else
      result = operand1 / operand2
      IO.puts("Result: #{result}")
    end
  end

  defp perform_operation(_, _,  _) do
    IO.puts("Error: Invalid operation")
  end
end

Calculator.main(System.argv())

