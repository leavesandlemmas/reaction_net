'''
Adapted from crafting interpreters
https://craftinginterpreters.com/scanning.html
'''
from enum import Enum 

class TokenType(Enum):

    # Single chracters 
    LEFT_PAREN = 1 
    RIGHT_PAREN = 2 
    PLUS = 3
    STAR = 4 
    COLON = 5
    SLASH = 12 


    # two character tokens
    RIGHT_ARROW=6
    LEFT_ARROW=7
    DOUBLE_ARROW=8
    EQUAL=9

    # literals 
    IDENTIFIER=10 
    NUMBER=11

    # keywords 
    WHERE = 13
    FOR = 14

    EOF = 0



class Token:
    
    def __init__(self, token_type, lexeme, literal, line):
        self.token_type = token_type
        self.lexeme = lexeme
        self.literal = literal
        self.line = line 

    def __repr__(self):
        return self.token_type + " " + self.lexeme + " " + self.literal


class Scanner:

    def __init__(self, source):
        self.source = source 
        self.tokens = []
        self.start = 0
        self.current = 0
        self.line = 1
    
    def scan_tokens(self):
        while not self.at_end():
            start = self.current 
            self.scan_token()
        
        self.tokens.append(Token(TokenType.EOF, "", None, self.line))
        return self.tokens 
    
    def at_end(self):
        return self.current >=  len(self.source)

    def scan_token(self):
        c = self.advance()
        match c:
            case "(" : 
                self.add_token(TokenType.LEFT_PAREN)
            case ")":
                self.add_token(TokenType.RIGHT_PAREN)
            case "+":
                self.add_token(TokenType.PLUS)
            case "*":
                self.add_token(TokenType.STAR)
            case ":":
                self.add_token(TokenType.COLON)
            case "-":
                if self.next_char_is(">"):
                    self.add_token(TokenType.RIGHT_ARROW)
            case "<":
                if self.next_char_is("-"):
                    self.add_token(TokenType.LEFT_ARROW)
            case "=":
                self.add_token(TokenType.EQUAL)
            case "/":
            
            case _ :
                raise Exception(
                    "Line: {}  Unexpected Character `{}`".format(self.line, c))
    
    def advance(self):
        self.current += 1
        return self.source[self.current - 1]

    def add_token(self, token_type, literal=None):
        text = self.source[slice(self.start, self.current)]
        self.tokens.append(Token(token_type, text, literal, self.line))

    def next_char_is(self, expect):
        if self.at_end(): 
            return False 
        if self.source[self.current] != expect:
            return False 
        self.current += 1
        return True 
        