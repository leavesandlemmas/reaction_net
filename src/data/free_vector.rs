use std::collections::HashMap; 
use std::hash::Hash;
use std::ops::{Add, Sub, Mul, Div};
// free vector space 
#[derive(Debug)]
pub struct FreeVector<T, K> where 
T : Eq + Hash + Clone, K : Add + Sub + Mul + Div
{
    terms: HashMap<T, K>,
}

impl<T : Eq + Hash + Clone, K : Add + Sub + Mul + Div> FreeVector<T, K> {
    pub fn new() -> Self {
        Self {
            terms: HashMap::new()
        }
    }

    pub fn monomial(&mut self, symbol : T, coef : K)
    {
        if let Some(&id) = self.index_map.get(&symbol) { 
            return id;
        }

        let new_id = self.symbols.len();

        self.index_map.insert(symbol.clone(), new_id);
        self.symbols.push(symbol);
        
        new_id
    }

    pub fn get_symbol(&self, idx : IdNum) -> &T {
        &self.symbols[idx]
    }
   
}
