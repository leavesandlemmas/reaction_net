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

    pub fn reversible(reactants: Complex, products: Complex) -> Self {
        Self {
            name: None,
            reversible: true,
            reactants,
            products,
        }
    }

    pub fn forward(reactants: Complex, products: Complex) -> Self {
        Self {
            name: None,
            reversible: false,
            reactants,
            products,
        }
    }

    pub fn named_reversible(name: String, reactants: Complex, products: Complex) -> Self {
        Self {
            name: Some(name),
            reversible: true,
            reactants,
            products,
        }
    }

    pub fn named_forward(name: String, reactants: Complex, products: Complex) -> Self {
        Self {
            name: Some(name),
            reversible: false,
            reactants,
            products,
        }
    }

    pub fn set_name(&mut self, name : String) {
        self.name = Some(name);
    }
    
    pub fn is_reversible(&self) -> bool {
        self.reversible
    }

    
    pub fn get_reactants(&self) -> &Complex {
        &self.reactants
    }


    pub fn get_products(&self) -> &Complex {
        &self.products
    }
}

