pub mod complex;
pub mod reaction;

use crate::data::Registry;

pub use complex::{Complex, SpeciesId, StoichCoef};
pub use reaction::Reaction;

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

    pub fn register_species(&mut self, s: String) -> SpeciesId {
        self.species.register(s)
    }

    pub fn add_reaction(&mut self, rxn: Reaction) {
        self.reactions.push(rxn);
    }

    pub fn add_term_to(&mut self, complex: &mut Complex, s: String, c: StoichCoef) {
        let id = self.register_species(s);
        complex.add_term(id, c);
    }
}
