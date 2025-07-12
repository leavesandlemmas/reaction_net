#include <iostream>
#include <fstream>
#include <string>

#include "include/rxn.h"

void report_error(
    int line, bool &error,
    const std::string &where, const std::string &msg)
{
    error = true;
    std::cout << "Line " << line << ":   Error\n   ";
    std::cout << where << ":   " << msg;
}

void raise_error(int line, bool &error, const std::string &msg)
{
    report_error(line, error, "", msg);
}

int main(int argc, char *argv[])
{
    if (argc > 1)
    {
        std::cout << "Reading first file.\n";

        Scanner<std::ifstream> scanner(argv[1]);

        scanner.scan_tokens();
        scanner.print_tokens();
    }
    else
    {
        std::cout << "No input passed.\n";
    }
}