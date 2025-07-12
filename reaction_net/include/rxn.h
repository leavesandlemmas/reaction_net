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
    EQUALS,

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
        return "Token=" + std::to_string(static_cast<int>(_type)) + "  " + _lexeme;
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

    void print_tokens()
    {
        for (auto &s : _tokens)
            std::cout << s.to_string() << '\n';
    }

private:
    StreamType _source;
    std::vector<Token> _tokens;
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
                "error";
            }
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
            add_token(TokenType::SEMICOLON, ";");
            ++_line;
            break;
        default:
            if (is_alphanumeric(c))
                identifier(c);
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

        while ((c = peek()) != ' ' && !at_end())
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
};