#include <iostream>
#include <fstream>
#include <string>

#include "include/rxn.h"

int main(int argc, char *argv[])
{
    if (argc > 1)
    {
        std::cout << "Reading first file.\n";

        Scanner<std::ifstream> scanner(argv[1]);

        scanner.scan_tokens();
        scanner.print_tokens(std::cout);
        if (scanner.any_errors())
        {
            std::cout << "Scanner encountered errors:\n";
            scanner.print_errors(std::cout);
        }
    }
    else
    {
        std::cout << "No input passed.\n";
    }
}