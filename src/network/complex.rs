use std::collections::HashMap;
use crate::data::registry::IdNum;
type SpeciesId = IdNum;
type StoichCoef = u64;
#[derive(Debug)]
pub struct Complex {
    terms: HashMap<SpeciesId, StoichCoef>,
}

impl Complex {
    pub fn new() -> Self {
        Self {
            terms: HashMap::new(),
        }
    }

//    pub fn monomial(network: &mut ReactionNet, s: &str) -> Self {
//        let mut terms = HashMap::new();
//        let id = network.register_species(s);
//        terms.insert(id, 1);
//        Self { terms }
//    }
//
//    pub fn monomial_with_coef(network: &mut ReactionNet, s: &str, c: StoichCoef) -> Self {
//        let mut terms = HashMap::new();
//        let id = network.register_species(s);
//        terms.insert(id, c);
//        Self { terms }
//    }
//
//    pub fn binomial(network: &mut ReactionNet, a: &str, b: &str) -> Self {
//        let mut terms = HashMap::new();
//        let a = network.register_species(a);
//        let b = network.register_species(b);
//        terms.insert(a, 1);
//        // check if b already present
//        terms.entry(b).and_modify(|x| *x += 1).or_insert(1);
//        Self { terms }
//    }
//
//    pub fn add_term(&mut self, network: &mut ReactionNet, s: &str, c: StoichCoef) {
//        let id = network.register_species(s);
//
//        self.terms.entry(id).and_modify(|x| *x += c).or_insert(c);
//    }
//}
}
