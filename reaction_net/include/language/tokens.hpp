#pragma once

#include <string>
// All terminal symbols in syntax
enum class TokenType
{
    // single characters
    LEFTPAREN,
    RIGHTPAREN,
    PLUS,
    STAR,
    SEMICOLON,
    COLON,
    EQUAL,
    GREATER,
    LESS,

    // one or two characters
    MINUS,
    SLASH,
    RIGHTARROW,
    LEFTARROW,
    LEFTRIGHTARROW,
    // literals
    SYMBOL,
    NUMBER,
    // keywords

    // end of file
    ENDOFFILE,

    // non terminal symbols
    reaction,
    complex,
    yield
};

class Token
{

    TokenType _type;
    std::string _lexeme;
    int _line;

public:
    Token(
        TokenType t,
        std::string lexeme, int line) : _type{t},
                                        _lexeme{lexeme},
                                        _line{line} {}

    inline std::string to_string()
    {
        return "L" + std::to_string(_line) + " Token=" + std::to_string(static_cast<int>(_type)) + "  " + _lexeme;
    }

    inline bool is_type(TokenType t) const
    {
        return _type == t;
    }
};