Run the application:

```
erlc cli_app.erl
```

Start Erlang shell and run it:


```
erl
Erlang/OTP 25 [erts-13.2.2.9] [source] [64-bit] [smp:8:4] [ds:8:4:10] [async-threads:1] [jit:ns]

Eshell V13.2.2.9  (abort with ^G)
1> cli_app:start().
```

Usage Examples:

```
Enter a command (type 'exit' to quit): hi
Hey, there!
Enter a command (type 'exit' to quit):
```

```
Enter a command (type 'exit' to quit): add 5 3
Result: 8
Enter a command (type 'exit' to quit):

```

```
Enter a command (type 'exit' to quit): add 5 3
Result: 8
Enter a command (type 'exit' to quit):
```

```
Enter a command (type 'exit' to quit): subtract 10 4
Result: 6
Enter a command (type 'exit' to quit):
```

```
Enter a command (type 'exit' to quit): multiply 7 6
Result: 42
Enter a command (type 'exit' to quit):
```

```
Enter a command (type 'exit' to quit): divide 20 4
Result: 5.0
Enter a command (type 'exit' to quit):
```

```
Enter a command (type 'exit' to quit): exit
Goodbye!
```
