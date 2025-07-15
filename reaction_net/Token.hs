module Token where

import Data.Char 

data Symbol = 
    -- one character symbols
    LEFTPAREN | RIGHTPAREN | PLUS |
    STAR | 
    SEMICOLON | 
    COLON | 
    EQUAL | 
    GREATER |
    LESS|
    MINUS | 
    SLASH | 
    
    -- two and three characters symbols
    RIGHTARROW | 
    LEFTARROW | 
    LEFTRIGHTARROW |

    -- identifiers 
    SYMBOL |
    NUMBER deriving (Eq, Show, Enum)

data Token = Token {
    tokenType :: Symbol,
    tokenLexeme :: String,
    tokenLine :: Int  
} deriving (Eq)

match s (c:cs) left right = if s == c then (left, cs) else (right, c:cs)

isSpecial c = elem c "()+*:;=-/\n"

tokenize [] = []
tokenize (c:cs) =
    | isSpecial c = matchSymbol c
    | isDigit c =
    | isAlphaNum c = 
    | isSpace c = tokenize cs 
    | otherwise =  error $ "Unexpected character: " ++ [c]
    

matchSymbol (c:cs) rem = case c of 
        '(' -> LEFTPAREN  : cs)  
        ')' -> (RIGHTPAREN, cs)
        '+' -> (PLUS, cs)
        '*' -> (STAR, cs)
        ':' -> (COLON, cs)
        ';' -> (SEMICOLON , cs)
        '=' -> (EQUAL, cs)
        '-' -> match '>' cs RIGHTARROW MINUS
        '<' -> match '-' cs (takeIfMatch '>' (advance cs) LEFTRIGHTARROW LEFTARROW) LESS
        '>' -> (GREATER, xs)
        '/' -> takeIfMatch '/' cs 
        
tokenize [] = []
