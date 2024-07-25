-module(cli_app).
-export([start/0]).

start() ->
	io:format("Welcome to the Enhanced Erlang CLI app!~n"),
	loop().

loop() -> 
	io:format("Enter a command (type 'exit' to quit): "),
	Input = io:get_line(""),
	Command = string:trim(Input),
	handle_command(Command).
