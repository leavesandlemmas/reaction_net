#pragma once

#include <sstream>
#include <string>
#include <vector>
#include <stdexcept>

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
    ENDOFFILE
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

struct Error
{
    size_t line;
    std::string ex;
    std::string msg;

    template <typename T1, typename T2, typename T3>
    Error(T1 l, T2 e, T3 m) : line{l}, ex{e}, msg{m}
    {
    }

    std::string report()
    {
        return "Error Line " + std::to_string(line) + ". " + msg + "\n  " + ex;
    }
};

template <typename StreamType>
class Scanner
{

public:
    template <typename... Args>
    Scanner(Args &&...args) : _source(std::forward<Args>(args)...), _tokens{}
    {
        if (!_source.good())
        {
            throw std::runtime_error("File failed to open...");
        }
    }

    void scan_tokens()
    {
        while (!at_end())
        {
            current_lexeme.clear();
            scan_token();
        }

        _tokens.push_back(
            Token(TokenType::ENDOFFILE, "", _line));
    }

    template <typename StreamOut>
    void print_tokens(StreamOut &out)
    {
        for (auto &s : _tokens)
            out << s.to_string() << '\n';
    }

    inline bool any_errors()
    {
        return errors.size() > 0;
    }

    template <typename StreamOut>
    inline void print_errors(StreamOut &out)
    {
        for (auto &s : errors)
        {
            out << s.report() << '\n';
        }
    }

private:
    StreamType _source;
    std::vector<Token> _tokens;
    std::vector<Error> errors;
    std::string current_lexeme;
    size_t _line = 1;

    void scan_token()
    {
        char c = advance();

        current_lexeme.push_back(c);
        switch (c)
        {
        case '(':
            add_token(TokenType::LEFTPAREN);
            break;
        case ')':
            add_token(TokenType::RIGHTPAREN);
            break;
        case '+':
            add_token(TokenType::PLUS);
            break;
        case '*':
            add_token(TokenType::STAR);
            break;
        case ';':
            add_token(TokenType::SEMICOLON);
            break;
        case ':':
            add_token(TokenType::COLON);
            break;
        case '=':
            add_token(TokenType::EQUAL);
            break;
        case '-':
            add_token(match('>') ? TokenType::RIGHTARROW : TokenType::MINUS);
            break;
        case '<':
            if (match('-'))
            {
                add_token(match('>') ? TokenType::LEFTRIGHTARROW : TokenType::RIGHTARROW);
            }
            else
            {
                add_token(TokenType::LESS);
            }
            break;
        case '>':
            add_token(TokenType::GREATER);
            break;
        case '/':
            if (match('/'))
            {
                newline();
            }
            else if (match('*'))
            {
                advance_to_comment_end();
            }

            else
            {
                add_token(TokenType::SLASH);
            }
            break;
        case ' ':
            break;
        case '\r':
            break;
        case '\t':
            break;
        case '\n':
            if (!_tokens.back().is_type(TokenType::SEMICOLON))
                add_token(TokenType::SEMICOLON, ";");
            ++_line;
            break;
        default:
            if (is_alphanumeric(c))
                identifier(c);
            else
                errors.push_back(Error(_line, current_lexeme, "Unrecognized character."));
        }
    }

    bool match(char expect)
    {
        char c = peek();
        if (c != expect)
        {
            return false;
        }
        current_lexeme.push_back(advance());
        return true;
    }

    void add_token(TokenType t)
    {
        _tokens.push_back(
            Token(t, current_lexeme, _line));
    }

    void add_token(TokenType t, std::string &&lexeme)
    {
        _tokens.push_back(
            Token(t, lexeme, _line));
    }

    inline char advance()
    {
        char c = _source.get();
        return c;
    }

    inline char peek()
    {
        if (at_end())
            return '\0';
        return _source.peek();
    }

    inline bool at_end()
    {
        return _source.peek() == EOF;
    }

    inline void newline()
    {
        while (peek() != '\n' && !at_end())
        {
            advance();
        }

        advance();
        ++_line;
    }

    inline void advance_to_comment_end()
    {
        while (!at_end())
        {
            if (advance() == '*' && peek() == '/')
                break;
        }
    }

    void identifier(char c)
    {
        bool digit_only = is_digit(c);

        while (!(is_identifier_end(c = peek()) || at_end()))
        {
            if (digit_only)
            {
                if (is_alphabet(c))
                    digit_only = false;
            }
            current_lexeme.push_back(advance());
        }
        if (digit_only)
        {
            add_token(TokenType::NUMBER);
        }
        else
        {
            add_token(TokenType::SYMBOL);
        }
    }

    inline bool is_alphanumeric(char c)
    {
        return is_digit(c) || is_alphabet(c);
    }

    inline bool is_alphabet(char c)
    {
        return ('a' <= c && c <= 'z') ||
               ('A' <= c && c <= 'Z') || c == '_';
    }

    inline bool is_digit(char c)
    {
        return ('0' <= c) && (c <= '9');
    }

    inline bool is_whitespace(char c)
    {
        return (c == ' ') || (c == '\n') || (c == '\t') || (c == '\r');
    }

    inline bool is_identifier_end(char c)
    {
        return !is_alphanumeric(c);
    }
};

class Parser
{
};