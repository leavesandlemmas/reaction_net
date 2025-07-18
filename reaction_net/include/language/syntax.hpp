#pragma once
#include <vector>
#include "tokens.hpp"

class Parser {
    std::vector<Token> tokens;
    size_t current = 0;

    public:
        Parser(std::vector<Token> ts) : tokens{ts} {}

    

    
};

struct Reaction
{
};
