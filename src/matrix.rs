use std::colections::HashMap;
// sparse matrix library


//dictionary of keys
pub struct DokMatrix<T> {
    data:HashMap<(usize, usize), T>,
    dim : Dim,    
}

impl<T> DokMatrix<T> {

    pub fn new() -> Self {
        let data = HashMap::new();
        let dim = Dim::new();
        Self {data, dim}
    }
    
    pub fn insert(&mut self, i : usize, j : usize, value : T) {
        let key = (i,j);   
        dim.update(i, j);
        data.entry(key)
            .and_modify(|e| *e += value)
            .or_insert(value);  
    } 
}


pub struct CooMatrix<T> {
    values : Vec<T>,
    row_index : Vec<usize>,
    col_index : Vec<usize>,
    dim : Dim,    
}


impl<T> CooMatrix<T> {
    
    pub fn new() -> Self {
        let dim = Dim::new();
        Self {val : Vec::new() , row : Vec::new(), col : Vec::new()} 
    }

    

}


struct Dim {
    row : usize,
    col : usize,
}

impl Dim {
 
    fn new() -> Self {
        Self{row:0, col:0}
    }

    pub fn row(&self) -> usize {
        self.row
    }
    
    
    pub fn col(&self) -> usize {
        self.col
    }

    pub fn update(&mut self, i : &usize, j : &usize ) {
        self.row = self.col.max(i);        
        self.col = self.col.max(j);
    }

}

#[cfg(test)]
mod tests {
    
}   
