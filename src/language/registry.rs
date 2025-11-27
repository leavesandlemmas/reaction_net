use std::collections::HashMap;

pub type IdNum = usize;
// data structure for labeling a set with indices
// used for symbol table and species registry
#[derive(Debug)]
pub struct Registry<T> where 
Hash<T> + Eq<T>
{
    index_map: HashMap<T, IdNum>,
    symbols: Vec<T>, // ??? make std::rc::Rc<String> instead of String ???
    //attributes: Vec<T>, 
}

impl<T : Hash<T> + Eq<T>> Registry<T> {
    pub fn new() -> Self {
        Self {
            index_map: HashMap::new(),
            symbols: Vec::new(),
            //attributes : Vec::new(),
        }
    }

    pub fn register<T>(&mut self, symbol: T) -> IdNum
    where
    {
        let s = symbol.as_ref();
        if let Some(&id) = self.index_map.get(s) { 
            return id;
        }

        let owned_s = symbol.into();
        let new_id = self.symbols.len();

        self.index_map.insert(owned_s.clone(), new_id);
        self.symbols.push(owned_s);
        //self.attributes.push(attribute);

        new_id
    }

    pub fn get_symbol(&self, idx : IdNum) -> &T {
        &self.symbols[idx]
    }
    
//    pub fn get_attribute(&self, idx : IdNum) -> &T {
//        &self.attributes[idx]
//    }
}
