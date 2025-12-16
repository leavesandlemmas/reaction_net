#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Complex {
    terms : Vec<(String, u64)>,
}


impl Complex {
    pub fn new() -> Self {
        Self { terms: Vec::new() }
    }

    pub fn add_term(&mut self, term: (String,  u64)) {
        self.terms.push(term);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arrow {
    Right,
    Left,
    Reversible,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reaction {
    name: Option<String>,
    arrow: Arrow,
    reactants: Complex,
    products: Complex,
}


impl Reaction {
    pub fn new(
        name: Option<String>,
        arrow: Arrow,
        reactants: Complex,
        products: Complex,
    ) -> Self {
        Self {
            name,
            arrow,
            reactants,
            products,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Network {
    reactions : Vec<Reaction>,
}

impl Network {
    pub fn new() -> Self {
        Self {reactions : Vec::new()}
    }

    pub fn add_reaction(&mut self, rxn : Reaction) {
        self.reactions.push(rxn);
    }
}