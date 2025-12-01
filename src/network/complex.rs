use std::collections::HashMap;
pub type SpeciesId = usize;
pub type StoichCoef = u64;
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

    pub fn add_term(&mut self, id: SpeciesId, coef: StoichCoef) {
        self.terms
            .entry(id)
            .and_modify(|x| *x += coef)
            .or_insert(coef);
    }
}
