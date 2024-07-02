-- File: Greet.hs
module Main where

import System.Environment (getArgs)

main :: IO ()
main = do
    args <- getArgs
    case args of
        [name] -> putStrLn $ "Hello, " ++ name ++ "!"
        _      -> putStrLn "Usage: greet <name>"
