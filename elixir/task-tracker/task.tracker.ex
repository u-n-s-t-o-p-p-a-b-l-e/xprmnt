defmodule TaskTracker do
  @moduledoc """
  A simple task tracker CLI app.
  """

  def main(args) do
    case args do
      ["add" | task] ->
        add_task(Enum.join(task, " "))
      ["list"] ->
        list_tasks()
      ["complete", id] ->
        complete_task(id)
      _ ->
        IO.puts("""
        Usage:
          mix run -e 'TaskTracker.main(["add", "Task Description"])'
          mix run -e 'TaskTracker.main(["list"])'
          mix run -e 'TaskTracker.main(["complete", "<task_id"])'
        """)
    end
  end

  defp add_task(task_description) do
    tasks = read_tasks()
    new_tasks = %{
      "id" => Enum.count(tasks) + 1,
      "description" => task_description,
      "complete" => false
    }
    updated_tasks = tasks ++ [new_task]
    write_tasks(updated_tasks)
    IO.puts("Added task: #{new_task["description"]}")
  end
