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
