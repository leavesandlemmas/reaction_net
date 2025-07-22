module Main (main) where 


import Text.Parsec

main = putStrLn "Hello World!"

-- adapted from https://hasura.io/blog/parser-combinators-walkthrough
-- newtype Parser a = Parser {
--     runParser :: String -> (String, Either ParseError a)
-- }

-- any :: Parser Char 
-- any = Parser $ \input -> case input of 
--     (x:xs) -> (xs, Right x)
--     [] -> ("", Left $ ParseError "any character" "the end of the input")

-- eof :: Parser ()
-- eof = Parser $ \input -> case input of 
--     [] -> ("", Right ())
--     (c:_) -> (input, Left $ ParseError "the end of the input" [c] )