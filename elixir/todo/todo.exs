defmodule Todo do
  @todo_file "todo_list.txt"

  def main(args) do
    case args do
      ["add",  task] ->
        add_task(task)
      ["list"]  ->
        list_tasks()
      ["complete", String.to_integer(index_str)]
        complete_task(index)
      _ ->
        IO.puts("""
          Usage:
          elixir todo.exs add <task>          - Add  a new task
          elixir todo.exs list                - List all tasks
          elixir todo.exs complete <index>    - Mark a task as complete
          """)
    end
  end

  defp add_task(task) do
    File.write!(@todo_file, "#{task}\n", [:append])
    IO.puts("Added task: #{task}")
  end

  defp list_tasks do
    if File.exists?(@todo_file)
      tasks = File.read!(@todo_file)
      |> String.split("\n", trim: true)

      tasks
      |> Enum.with_index()
      |> Enum.each(fn {task, index} ->
        IO.puts("#{index + 1}. #{task}")
      end)
    else
      IO.puts("No tasks found.")
    end
end
