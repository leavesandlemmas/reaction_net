use crate::registry::Registry;
use super::*;


#[derive(Debug, Clone)]
pub struct NetworkBuilder {
    species_registry: Registry<String>,
    complex_matrix: CscMatrix<Coef>,
    incidence_matrix: CooMatrix<Coef>,
}

impl NetworkBuilder {

    pub fn new() -> Self{
        Self {
            species_registry: Registry::new(),
            complex_matrix: CscMatrix::new(),
            incidence_matrix: CooMatrix::new(),
        }
    }

    // parser commands
    pub fn register_species(&mut self, species: String) -> usize {
        self.species_registry.register(species)
    }

    pub fn add_term_to_complex(&mut self, cplx: &mut Complex, species: String, coef: Coef) {
        let s = self.register_species(species);
        cplx.insert(s, coef);
    }


    pub fn add_forward_reaction(&mut self, mut reactants: Complex, mut products: Complex) {
        reactants.canonical_format();
        products.canonical_format();
        let r = self.complex_matrix.append_column_if_unique(reactants);
        let p = self.complex_matrix.append_column_if_unique(products);
        let reaction_num = self.incidence_matrix.ncol();
        self.incidence_matrix.insert(r, reaction_num, -1);
        self.incidence_matrix.insert(p, reaction_num,  1);
    }

    pub fn add_reversible_reaction(&mut self, mut reactants: Complex, mut products: Complex) {
        reactants.canonical_format();
        products.canonical_format();
        let r = self.complex_matrix.append_column_if_unique(reactants);
        let p = self.complex_matrix.append_column_if_unique(products);
        let reaction_num = self.incidence_matrix.ncol();
        self.incidence_matrix.insert(r, reaction_num, -1);
        self.incidence_matrix.insert(p, reaction_num,  1);
        self.incidence_matrix.insert(r, reaction_num + 1,  1);
        self.incidence_matrix.insert(p, reaction_num + 1,  -1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_one_reaction_network() {
        //X + Y -> Z 
        let mut net = NetworkBuilder::new();

        let mut reactant = Complex::new();
        net.add_term_to_complex(&mut reactant, "X".to_string(), 1);
        net.add_term_to_complex(&mut reactant, "Y".to_string(), 1);

        let mut product = Complex::new();        
        net.add_term_to_complex(&mut product, "Z".to_string(), 1);
        
        net.add_forward_reaction(reactant, product);
              
        
            
    }   
    

}
