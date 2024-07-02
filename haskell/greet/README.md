Compile the program using GHC (Glasgow Haskell Compiler):

```
ghc --make Greet.hs -o greet
```

Run the compiled program with name argument:
```
./greet John
```

This should output:
```
Hello, John!
```

Run the program without an argument or with more than one argument, it will display the usage message:

```
./greet
Usage: greet <name>
```
