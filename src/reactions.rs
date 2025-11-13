use std::collections::HashMap;

struct Complex {
    terms: HashMap<str, usize>,
}


struct Reaction {
    name : &str,
    reactants: Complex,
    products: Complex,
}
