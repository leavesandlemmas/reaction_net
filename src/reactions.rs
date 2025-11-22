use std::collections::HashMap;

// memory efficient to copy
type StoichCoef = u32;
type SpeciesID = usize;

// ensures species have unique names
#[derive(Debug)]
struct SpeciesRegistry {
    idmap: HashMap<String, SpeciesID>,
    names: Vec<String>,
}

impl SpeciesRegistry {
    
    fn new() -> Self {
        Self {idmap : HashMap::new(), names : Vec::new()}
    }

    fn register<S>(&mut self, species : S) -> SpeciesID 
    where 
        S:Into<String> + AsRef<str>,
    {
       let s = species.as_ref();
       if let Some(&id) = self.idmap.get(s) {
        return id;
       } 

       let owned_s = species.into();
       let new_id = self.names.len();

       self.idmap.insert(owned_s.clone(), new_id);
       self.names.push(owned_s);

       new_id
    }

    fn get_name(&self, id : SpeciesID) -> &str {
        &self.names[id]
    }
}

#[derive(Debug)]
struct Complex {
    terms: HashMap<SpeciesID, StoichCoef>,
}

impl Complex {
    
    fn new() -> Self {
        Self {terms : HashMap::new()}    
    }

    fn monomial(registry : & mut SpeciesRegistry, s : &str) -> Self {
        let mut terms = HashMap::new();
        let id = registry.register(s);
        terms.insert(id, 1);
        Self {terms}
    }

    fn monomial_with_coef(registry : & mut SpeciesRegistry, s : &str, c : StoichCoef) -> Self {
        let mut terms = HashMap::new();
        let id = registry.register(s);
        terms.insert(id, c);
        Self {terms}
    }

    fn binomial(registry : & mut SpeciesRegistry, a : &str, b : &str) -> Self {
        let mut terms = HashMap::new();
        let a = registry.register(a);
        let b = registry.register(b);
        terms.insert(a, 1);
        // check if b already present
        terms.entry(b)
        .and_modify(|x| *x += 1)
        .or_insert(1);
        Self {terms}
    }
    
    fn add_term(&mut self, registry : & mut SpeciesRegistry, s : &str, c : StoichCoef) {
        let id = registry.register(s);

        self.terms.entry(id)
        .and_modify(|x| *x += c)
        .or_insert(c);
    }

}

#[derive(Debug)]
struct Reaction {
    name : Option<String>,
    reversible : bool, 
    reactants: Complex,
    products: Complex,
}

impl Reaction {

    fn new() -> Self {
        Self {
            name : None, 
            reversible : false,
            reactants : Complex::new(), 
            products : Complex::new(),
        }
    }
    
    fn reversible(reactants : Complex, products : Complex) -> Self {
        Self {name : None, reversible:true, reactants, products}
    }

    fn forward(reactants : Complex, products : Complex) -> Self {
        Self {name : None, reversible : false, reactants, products}
    }

    fn named_reversible(name : String, reactants : Complex, products : Complex) -> Self {
        Self {name : Some(name), reversible:true, reactants, products}
    }

    fn named_forward(name : String, reactants : Complex, products : Complex) -> Self {
        Self {name : Some(name), reversible:false, reactants, products}
    }
    
}

#[derive(Debug)]
pub struct RxNet{
    registry : SpeciesRegistry,
    reactions : Vec<Reaction>,
}

impl RxNet {

    fn new() -> Self {
        Self { registry : SpeciesRegistry::new(), reactions : Vec::new()}
    }

    pub fn build_example() -> Self {
        let mut registry = SpeciesRegistry::new();
        let mut reactions : Vec<Reaction> = Vec::new();
        let rct = Complex::binomial(&mut registry, "A", "C1");
        let prod = Complex::binomial(&mut registry, "B", "C2");
        let rxn = Reaction::forward(rct, prod);
        reactions.push(rxn);
        
        let rct = Complex::monomial(&mut registry, "C2");
        let prod = Complex::monomial(&mut registry, "C3");
        let rxn = Reaction::forward(rct, prod);
        reactions.push(rxn);

        
        let rct = Complex::monomial(&mut registry, "C3");
        let prod = Complex::monomial(&mut registry, "C1");
        let rxn = Reaction::forward(rct, prod);
        reactions.push(rxn);

        let rct = Complex::binomial(&mut registry, "S", "E");
        let prod = Complex::monomial(&mut registry, "ES");
        let rxn = Reaction::reversible(rct, prod);
        reactions.push(rxn);

        let rct = Complex::monomial(&mut registry, "ES");
        let prod = Complex::binomial(&mut registry, "E", "P");
        let rxn = Reaction::forward(rct, prod); 
        reactions.push(rxn);     
        Self {registry, reactions}
    }

}
