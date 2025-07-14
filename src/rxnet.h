#pragma once

#include <string>
#include <vector>


struct Species {
    
    std::string name;

    double conc;
    double dt;

    Species(std::string n) : name{n}, conc{0}, dt{0} {}

};

struct Reaction {

    struct Edge {
        Species& species;
        size_t coef;
        
        Edge(const Edge& e) : species{e.species}, coef{e.coef} {}
        Edge(Species& s) : species{s}, coef{1} {}
        Edge(Species& s, size_t n) : species{s}, coef{n} {}
    };
    
    std::string name;
    std::vector<Edge> reactants;
    std::vector<Edge> products;

    



};
