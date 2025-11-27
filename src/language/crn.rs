use std::collections::HashMap;
use crate::language::registry::Registry;
// memory efficient to copy
type StoichCoef = u64;
type SpeciesID = usize;

// ensures species have unique names
//#[derive(Debug)]
//pub struct SpeciesRegistry {
//    idmap: HashMap<String, SpeciesID>,
//    names: Vec<String>,
//}
//
//impl SpeciesRegistry {
//    pub fn new() -> Self {
//        Self {
//            idmap: HashMap::new(),
//            names: Vec::new(),
//        }
//    }
//
//    pub fn register<S>(&mut self, species: S) -> SpeciesID
//    where
//        S: Into<String> + AsRef<str>,
//    {
//        let s = species.as_ref();
//        if let Some(&id) = self.idmap.get(s) {
//            return id;
//        }
//
//        let owned_s = species.into();
//        let new_id = self.names.len();
//
//        self.idmap.insert(owned_s.clone(), new_id);
//        self.names.push(owned_s);
//
//        new_id
//    }
//
//    pub fn get_name(&self, id: SpeciesID) -> &str {
//        &self.names[id]
//    }
//}
//
#[derive(Debug)]
pub struct Complex {
    terms: HashMap<SpeciesID, StoichCoef>,
}

impl Complex {
    pub fn new() -> Self {
        Self {
            terms: HashMap::new(),
        }
    }

    pub fn monomial(network: &mut ReactionNet, s: &str) -> Self {
        let mut terms = HashMap::new();
        let id = network.register(s);
        terms.insert(id, 1);
        Self { terms }
    }

    pub fn monomial_with_coef(network: &mut ReactionNet, s: &str, c: StoichCoef) -> Self {
        let mut terms = HashMap::new();
        let id = registry.register(s);
        terms.insert(id, c);
        Self { terms }
    }

    pub fn binomial(network: &mut ReactionNet, a: &str, b: &str) -> Self {
        let mut terms = HashMap::new();
        let a = registry.register(a);
        let b = registry.register(b);
        terms.insert(a, 1);
        // check if b already present
        terms.entry(b).and_modify(|x| *x += 1).or_insert(1);
        Self { terms }
    }

    pub fn add_term(&mut self, network: &mut ReactionNet, s: &str, c: StoichCoef) {
        let id = registry.register(s);

        self.terms.entry(id).and_modify(|x| *x += c).or_insert(c);
    }
}

#[derive(Debug)]
pub struct Reaction {
    name: Option<String>,
    reversible: bool,
    reactants: Complex,
    products: Complex,
}

impl Reaction {
    pub fn new() -> Self {
        Self {
            name: None,
            reversible: false,
            reactants: Complex::new(),
            products: Complex::new(),
        }
    }

    fn reversible(reactants: Complex, products: Complex) -> Self {
        Self {
            name: None,
            reversible: true,
            reactants,
            products,
        }
    }

    fn forward(reactants: Complex, products: Complex) -> Self {
        Self {
            name: None,
            reversible: false,
            reactants,
            products,
        }
    }

    fn named_reversible(name: String, reactants: Complex, products: Complex) -> Self {
        Self {
            name: Some(name),
            reversible: true,
            reactants,
            products,
        }
    }

    fn named_forward(name: String, reactants: Complex, products: Complex) -> Self {
        Self {
            name: Some(name),
            reversible: false,
            reactants,
            products,
        }
    }
}

pub type SpeciesRegistry = Registry<String>;
#[derive(Debug)]
pub struct ReactionNet {
    species: SpeciesRegistry,
    reactions: Vec<Reaction>,
}

impl ReactionNet {
    pub fn new() -> Self {
        Self {
            species: SpeciesRegistry::new(),
            reactions: Vec::new(),
        }
    }

    pub fn add_reaction(&mut self, rxn : Reaction){
        self.reactions.push(rxn);
    }

    pub fn register_species<S>(&mut self, s : S) -> IdNum where
        S : Into<String> + AsRef<str>
    {   
        self.species.register(s)
    }
    
    
    pub fn build_example() -> Self {
        let mut network = SpeciesRegistry::new();
        let rct = Complex::binomial(&mut network, "A", "C1");
        let prod = Complex::binomial(&mut network, "B", "C2");
        let rxn = Reaction::forward(rct, prod);
        network.add_reaction(rxn);

        let rct = Complex::monomial(&mut network, "C2");
        let prod = Complex::monomial(&mut network, "C3");
        let rxn = Reaction::forward(rct, prod);
        network.add_reaction(rxn);

        let rct = Complex::monomial(&mut network, "C3");
        let prod = Complex::monomial(&mut network, "C1");
        let rxn = Reaction::forward(rct, prod);
        network.add_reaction(rxn);

        let rct = Complex::binomial(&mut network, "S", "E");
        let prod = Complex::monomial(&mut network, "ES");
        let rxn = Reaction::reversible(rct, prod);
        network.add_reaction(rxn);

        let rct = Complex::monomial(&mut network, "ES");
        let prod = Complex::binomial(&mut network, "E", "P");
        let rxn = Reaction::forward(rct, prod);
        network.add_reaction(rxn);
        network
    }
    
}
