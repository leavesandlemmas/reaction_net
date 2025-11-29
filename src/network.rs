pub mod reaction;
pub mod complex;

use crate::data::{Registry, IdNum};

use reaction::Reaction;
type SpeciesId = IdNum;

pub type SpeciesRegistry = Registry<String>;
#[derive(Debug)]
pub struct Network {
    species: SpeciesRegistry,
    reactions: Vec<Reaction>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            species: SpeciesRegistry::new(),
            reactions: Vec::new(),
        }
    }

    pub fn add_reaction(&mut self, rxn : Reaction){
        self.reactions.push(rxn);
    }

//    pub fn register_species<S>(&mut self, s : S) -> SpeciesId where
//        S : Into<String> + AsRef<str>
//    {   
//        self.species.register(s)
//    }
//    
    
//    pub fn build_example() -> Self {
//        let mut network = SpeciesRegistry::new();
//        let rct = Complex::binomial(&mut network, "A", "C1");
//        let prod = Complex::binomial(&mut network, "B", "C2");
//        let rxn = Reaction::forward(rct, prod);
//        network.add_reaction(rxn);
//
//        let rct = Complex::monomial(&mut network, "C2");
//        let prod = Complex::monomial(&mut network, "C3");
//        let rxn = Reaction::forward(rct, prod);
//        network.add_reaction(rxn);
//
//        let rct = Complex::monomial(&mut network, "C3");
//        let prod = Complex::monomial(&mut network, "C1");
//        let rxn = Reaction::forward(rct, prod);
//        network.add_reaction(rxn);
//
//        let rct = Complex::binomial(&mut network, "S", "E");
//        let prod = Complex::monomial(&mut network, "ES");
//        let rxn = Reaction::reversible(rct, prod);
//        network.add_reaction(rxn);
//
//        let rct = Complex::monomial(&mut network, "ES");
//        let prod = Complex::binomial(&mut network, "E", "P");
//        let rxn = Reaction::forward(rct, prod);
//        network.add_reaction(rxn);
//        network
//    }
    
}



