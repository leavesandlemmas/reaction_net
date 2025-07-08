#include "rxnet.h"

int main(){

    Species A("A");
    Species B('B');
    Species C("C");
    
    Reaction r{'1'};
    r.reactants = {Reaction::Edge(A, 1)};
    r.products = {Reaction::Edge(B, 1), Reaction::Edge(C, 1)}; 
    
}