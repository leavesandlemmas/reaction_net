use super::complex::Complex;
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

