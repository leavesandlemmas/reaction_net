use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::ops::Index;

// data structure for labeling a set with indices
/// owns T (and copies)
pub type Registry<T> = RegistryImpl<T>; 
/// doesnt own T (copies poitners)
pub type RcRegistry<T> = RegistryImpl<Rc<T>>; 
// used for symbol table and species registry
#[derive(Debug, Clone)]
pub struct RegistryImpl<T>
where
    T: Eq + Hash + Clone,
{
    index_map: HashMap<T, usize>,
    symbols: Vec<T>,
}

impl<T: Eq + Hash + Clone> Registry<T> {
    pub fn new() -> Self {
        Self {
            index_map: HashMap::new(),
            symbols: Vec::new(),
        }
    }

    pub fn register(&mut self, symbol: T) -> usize {
        if let Some(&id) = self.index_map.get(&symbol) {
            return id;
        }

        let new_id = self.symbols.len();

        self.index_map.insert(symbol.clone(), new_id);
        self.symbols.push(symbol);

        new_id
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.symbols.get(idx)
    }

}

impl<T : Eq + Hash + Clone> Index<usize> for Registry<T> {
    type Output = T;

    fn index(&self, idx : usize) -> &Self::Output {
        &self.symbols[idx]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_registry() {
        let mut registry: Registry<String> = Registry::new();
        let idx = registry.register("A".to_string());
        assert_eq!(idx, 0);
        let idx = registry.register("B".to_string());
        assert_eq!(idx, 1);

        let idx = registry.register("A".to_string());
        assert_eq!(idx, 0);

        assert_eq!(registry.get(3), None);
        
    }

    
}
