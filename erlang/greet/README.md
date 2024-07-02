Compile it with erlang shell:

```
erlc greet.erl
```

Start the erlang shell in terminal and run this CLI application:
```
erl
> greet:start().
```

This will appear:
```
Welcome to the Erlang CLI app!
Enter a command (type 'exit' to quit): 
```

Enter commands such as hello, exit, or any other command. The app will respond accordingly:

```
Enter a command (type 'exit' to quit): hello
Hello, world!
Enter a command (type 'exit' to quit): exit
Goodbye!

```
