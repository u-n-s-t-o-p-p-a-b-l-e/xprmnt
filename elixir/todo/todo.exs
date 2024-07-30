defmodule Todo do
  @todo_file "todo_list.txt"

  def main(args) do
    case args do
      ["add",  task] ->
        add_task(task)
      ["list"]  ->
        list_tasks()
      ["complete", index_str] ->
        index = String.to_integer(index_str)
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
    if File.exists?(@todo_file) do
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

  defp complete_task(index) do
    if File.exists?(@todo_file) do
      tasks = File.read!(@todo_file)
      |> String.split("\n", trim: true)

      case Enum.at(tasks, index - 1) do
        nil ->
          IO.puts("Invalid task index.")
        task ->
          updated_tasks = List.delete_at(tasks, index - 1)
          File.write!(@todo_file, Enum.join(updated_tasks, "\n"))
          IO.puts("Completed task: #{task}")
      end
    else
        IO.puts("No tasks found.")
    end
  end
end

Todo.main(System.argv())
