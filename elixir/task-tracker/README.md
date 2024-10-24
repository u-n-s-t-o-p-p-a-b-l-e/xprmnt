Create Mix Project:

```
mix new task_tracker
```

Place mix.exs in the directory created.
run this to import jason dependencies:

```
mix deps.get
```

Replace lib/task_tracker.ex with the current file.
Run it to add task:

```
mix run -e 'TaskTracker.main(["add", "Buy groceries"])'
```

List Tasks:

```
mix run -e 'TaskTracker.main(["list"])'
```

Complete a Task:

```
mix run -e 'TaskTracker.main(["complete", "1"])'
```
