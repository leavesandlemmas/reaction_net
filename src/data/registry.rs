use std::collections::HashMap; 
use std::hash::Hash;
pub type IdNum = usize;
// data structure for labeling a set with indices
// used for symbol table and species registry
#[derive(Debug)]
pub struct Registry<T> where 
T : Eq + Hash + Clone
{
    index_map: HashMap<T, IdNum>,
    symbols: Vec<T>, // ??? make std::rc::Rc<String> instead of String ???
    //attributes: Vec<T>, 
}

impl<T : Eq + Hash + Clone> Registry<T> {
    pub fn new() -> Self {
        Self {
            index_map: HashMap::new(),
            symbols: Vec::new(),
            //attributes : Vec::new(),
        }
    }

    pub fn register(&mut self, symbol: T) -> IdNum
    where
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
