use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CscMatrix<T> {
    values: Vec<T>,
    col_ptr: Vec<usize>,
    row_indices: Vec<usize>,
    nrow: usize,
    ncol: usize,
}

impl<T: Scalar> CscMatrix<T> {
    
    pub fn new() -> Self{
        Self {
            values: Vec::new(),
            col_ptr: vec![0],
            row_indices: Vec::new(),
            nrow: 0,
            ncol: 0,
        }
    }

    pub fn from_raw(
        values: Vec<T>, 
        col_ptr: Vec<usize>, 
        row_indices: Vec<usize>, 
        nrow : usize, 
        ncol: usize ) -> Self {
            assert_eq!(col_ptr.len(), ncol + 1);
            assert_eq!(values.len(), row_indices.len());
            let min_nrow = row_indices.iter().max().unwrap() + 1;
            assert!(nrow >= min_nrow);
            Self{values, col_ptr, row_indices, nrow, ncol}

    }

    pub fn into_raw(self) -> (Vec<T>, Vec<usize>, Vec<usize>, usize, usize) {
        (self.values, self.col_ptr, self.row_indices, self.nrow, self.ncol)
    }

    pub fn view_raw(&self) -> (&[T], &[usize], &[usize]) {
        (self.values.as_ref(), self.col_ptr.as_ref(), self.row_indices.as_ref())
    }

    pub fn get_col_index(&self, colvec: &CompressedVector<T>) -> Option<usize> {
        // ensure sorted and no duplicates
        assert!(colvec.is_canonical_format());

        // if matrix is empty, then no columns to match        
        if self.ncol == 0 {
            return None; 
        }                     
        
        // loop through columns
        let vec_nnz = colvec.nnz();
        let (values, indices) = colvec.view_raw();        
        'outer: for c in 0..self.ncol() {
            // check if nnz 
            let col_nnz = self.col_ptr[c+1] - self.col_ptr[c];
            if col_nnz != vec_nnz {
                continue;
            }
            // check values and row_indices match
            let start = self.col_ptr[c]; 
            for r in 0..col_nnz {
                if self.values[start + r] != values[r] || self.row_indices[start + r] != indices[r] {
                    continue 'outer;
                } 
            }

            return Some(c);             
        }

        None 
    }

    pub fn append_column_if_unique(&mut self, colvec: CompressedVector<T>) -> usize {
        let idx = self.get_col_index(&colvec);      
        match idx {
            Some(c) => return c,
            _ => (), 
        };        

        
                
        // append column
        let (mut values, mut indices, dim) = colvec.into_raw();
        self.values.append(&mut values);
        self.row_indices.append(&mut indices);
        self.col_ptr.push(self.nnz());
        
        self.ncol += 1;
        self.nrow = self.nrow.max(dim);
        self.ncol - 1    
    }

}

impl<T>  SparseMatrix for CscMatrix<T>  {
    fn nrow(&self) -> usize {
        self.nrow 
    }
    
    fn ncol(&self) -> usize {
        self.ncol
    }

    fn nnz(&self) -> usize {
        self.values.len()
    }
} 



#[cfg(test)]
mod tests {
    use super::*;
    
    fn build_simple_csc() -> CscMatrix<i32> {
        // A + B -> C + D
        // C <-> E + F
        // 2 *A + B <-> G 
        let values = vec![1,1,1,1,1,1,1,2,1,1];
        let col_ptr = vec![0, 2, 4, 5, 7, 9, 10];
        let row_indices = vec![0,1,2,3,2,4,5,0,1,6];

        CscMatrix::from_raw(values, col_ptr, row_indices, 7, 6)    
    }

 
    #[test]
    fn unique_col_index() {
        let csc = build_simple_csc();
        
        let mut colvec = CompressedVector::new();
        colvec.insert(0, 1); // A
        colvec.insert(1, 1); // B
        colvec.canonical_format();
        assert_eq!( csc.get_col_index(&colvec), Some(0));        

    }

    
    #[test]
    fn unique_col_index_2() {
        let csc = build_simple_csc();
        
        let mut colvec = CompressedVector::new();
        colvec.insert(0, 2); // 2 * A
        colvec.insert(1, 1); // B
        colvec.canonical_format();
        
        assert_eq!( csc.get_col_index(&colvec), Some(4));        

    }

    
    #[test]
    fn unique_col_index_3() {
        let csc = build_simple_csc();
        let mut colvec = CompressedVector::new();
        colvec.insert(0, 1); // A
        colvec.insert(1, 2); // 2 *B
        colvec.canonical_format();
        
        assert_eq!( csc.get_col_index(&colvec), None);        

    }

    #[test]
    fn build_by_append() {
        let mut csc : CscMatrix<i32> = CscMatrix::new();
        // A + B -> C + D
        // C <-> E + F
        // 2 *A + B <-> G
        let mut colvec = CompressedVector::new();
        colvec.insert(0, 1); // A
        colvec.insert(1, 1); // B
        colvec.canonical_format();
        let i = csc.append_column_if_unique(colvec);
        assert_eq!(i , 0);
    
        let mut colvec = CompressedVector::new();
        colvec.insert(2, 1); // C
        colvec.insert(3, 1); // D
        colvec.canonical_format();
        let _ = csc.append_column_if_unique(colvec);        

        let mut colvec = CompressedVector::new();
        colvec.insert(2, 1); // C
        colvec.canonical_format();
        let _ = csc.append_column_if_unique(colvec);        
        
        let mut colvec = CompressedVector::new();
        colvec.insert(4, 1); // E
        colvec.insert(5, 1); // F
        colvec.canonical_format();
        let _ = csc.append_column_if_unique(colvec);        

        let mut colvec = CompressedVector::new();
        colvec.insert(0, 2); // 2 * A
        colvec.insert(1, 1); // B
        colvec.canonical_format();
        let _ = csc.append_column_if_unique(colvec);        

        let mut colvec = CompressedVector::new();
        colvec.insert(6, 1); // G
        colvec.canonical_format();
        let _ = csc.append_column_if_unique(colvec);        

        assert_eq!(csc, build_simple_csc());
    }

    
}
