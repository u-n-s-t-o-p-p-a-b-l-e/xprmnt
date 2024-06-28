This demonstrates how to use Goroutines and channels to handle concurrent tasks in a Go CLI application.


```
go run task-processor.go
```

Example input and output:

```
Enter tasks in the format 'name duration(s)' (e.g., 'task1 2s'). Enter 'exit' to finish.
task1 2s
task2 3s
task3 1s
exit
Worker 1: Processing task task1
Worker 2: Processing task task2
Worker 3: Processing task task3
Worker 3: Finished task task3
Worker 1: Finished task task1
Worker 2: Finished task task2
All tasks processed. Exiting.
```

 
