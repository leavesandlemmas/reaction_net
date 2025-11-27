use std::collections::HashMap;

pub type IdNum = usize;
// data structure for labeling a set with indices
// used for symbol table and species registry
pub struct Registry {
    index_map: HashMap<String, IdNum>,
    symbols: Vec<String>, // ??? make std::rc::Rc<String> instead of String ???
    //attributes: Vec<T>, 
}

impl Registry {
    pub fn new() -> Self {
        Self {
            index_map: HashMap::new(),
            symbols: Vec::new(),
            //attributes : Vec::new(),
        }
    }

    pub fn register<S>(&mut self, species: S) -> IdNum
    where
        S: Into<String> + AsRef<str>,
    {
        let s = species.as_ref();
        if let Some(&id) = self.index_map.get(s) { 
            return id;
        }

        let owned_s = species.into();
        let new_id = self.symbols.len();

        self.index_map.insert(owned_s.clone(), new_id);
        self.symbols.push(owned_s);
        //self.attributes.push(attribute);

        new_id
    }

    pub fn get_symbol(&self, idx : IdNum) -> &str {
        &self.symbols[idx]
    }
    
//    pub fn get_attribute(&self, idx : IdNum) -> &T {
//        &self.attributes[idx]
//    }
}
