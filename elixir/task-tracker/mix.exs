defmodule TaskTracker.MixProject do
  use Mix.Project

  def project do
    [
      app: :task_tracker,
      version: "0.1.0",
      elixir: "~> 1.14",
      deps: deps()
    ]
  end

  defp deps do
    [
      {:jason, "~> 1.4"}
    ]
  end
end
