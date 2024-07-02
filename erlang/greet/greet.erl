-module(cli_app).
-export([start/0]).

start() ->
	io:format("Welcome to the Erlang CLI app!~n"),
	loop().

loop() ->
	io:format("Enter a command (type 'exit' to quit): "),
	Input = io:get_line(""),
	Command = string:trim(Input),
	case Command of
		"exit" ->
			io:format("Goodbye!~n");
		"hello" ->
			io:format("Hello, world!~n"),
			loop();
		_ ->
			io:format("Unknown command: ~s~n", [Command]),
			loop()
	end.
