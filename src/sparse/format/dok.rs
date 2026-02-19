use std::collections::HashMap;
use super::*;

// dictionary of keys
// + element access
pub struct DokMatrix<T> {
    nrow: usize,
    ncol: usize,
    data: HashMap<(usize, usize), T>,
}

impl<T: Scalar> DokMatrix<T> {
    pub fn new() -> Self {
        let data = HashMap::new();
        Self { data, nrow:0, ncol: 0}
    }

    pub fn into_raw(self) -> (HashMap<(usize, usize), T>, usize, usize) {
        (self.data, self.nrow, self.ncol)
    }

    pub fn insert(&mut self, row: usize, col: usize, value: T) {
        let key = (row, col);
        self.nrow = self.nrow.max(row + 1); 
        self.ncol = self.ncol.max(col + 1);
        self.data
            .entry(key)
            .and_modify(|e| *e += value.clone())
            .or_insert(value);
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        let key = (row, col);
        self.data.get(&key)
    }

}

impl<T>  SparseMatrix for DokMatrix<T>  {
    fn nrow(&self) -> usize {
        self.nrow 
    }
    
    fn ncol(&self) -> usize {
        self.ncol
    }
    
    fn nnz(&self) -> usize {
        self.data.len()
    }
} 



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_dok_matrix() {
        let mut mat: DokMatrix<i64> = DokMatrix::new();
        assert_eq!(mat.nrow(), 0);
        assert_eq!(mat.ncol(), 0);
        mat.insert(0, 0, 1);

        assert_eq!(mat.nrow(), 1);
        assert_eq!(mat.ncol(), 1);

        mat.insert(0, 1, -1);
        assert_eq!(mat.nrow(), 1);
        assert_eq!(mat.ncol(), 2);
        mat.insert(0, 0, 1); // should be two
        let val = mat.get(0, 0);
        assert!(val.is_some());
        assert_eq!(val.unwrap().clone(), 2);

        mat.insert(7, 5, 5);
        assert_eq!(mat.nrow(), 8);
        assert_eq!(mat.ncol(), 6);
    }
}