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

handle_command("exit") ->
	io:format("Goodbye!~n");
handle_command("hi") ->
	io:format("Hey, there!~n"),
	loop();
handle_command(Command) ->
	case string:tokens(Command, " ") of
		["add", Num1Str, Num2Str] ->
			handle_math_command(Num1Str, Num2Str, fun erlang:'+'/2);
		["substract", Num1Str, Num2Str] ->
			handle_math_command(Num1Str, Num2Str, fun erlang:'-'/2);
		["multiply", Num1Str, Num2Str] ->
			handle_math_command(Num1Str, Num2Str, fun erlang:'*'/2);
		["divide", Num1Str, Num2Str] ->
			handle_math_command(Num1Str, Num2Str, fun erlang:'/'/2);
		_ ->
			io:format("Unknown command: ~s~n", [Command]),
			loop()
	end.

handle_math_command(Num1Str, Num2Str, Operation) ->
	Num1 = string_to_number(Num1Str),
	Num2 = string_to_number(Num2Str),
	case {Num1, Num2} of
		{{ok, N1}, {ok, N2}} ->
			Result = Operation(N1, N2),
			io:format("Result: ~p~n", [Result]),
			loop();
		_ ->
			io:format("Invalid numbers: ~s ~s~n", [Num1Str, Num2Str]),
			loop()
	end.
