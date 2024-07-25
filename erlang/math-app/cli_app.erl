-module(cli_app).
-export([start/0]).

start() ->
	io:format("Welcome to the Enhanced Erlang CLI app!~n"),
	loop().
