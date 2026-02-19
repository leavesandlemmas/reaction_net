use super::*;

#[derive(Debug, Clone)]
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
            col_ptr: Vec::new(),
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

    pub fn into_raw(self) ->(Vec<T>, Vec<usize>, Vec<usize>, usize, usize) {
        (self.values, self.col_ptr, self.row_indices, self.nrow, self.ncol)
    }

    pub fn view_raw(&self) -> (&[T], &[usize], &[usize]) {
        (self.values.as_ref(), self.col_ptr.as_ref(), self.row_indices.as_ref())
    }

    pub fn insert_col_if_unique(&mut self, col: CompressedVector<T>) -> usize {
        0
    }

//    pub fn get_col_index(&self, values : &[T], row_indices : &[usize]) -> Option<usize> {
//        if self.col_ptr.len() == 0 {
//            return None;        
//        }             
//        
//        // Check each column
//        for col_idx in 0..self.ncol {
//            if self.column_matches(col_idx, values, row_indices) {
//                return Some(col_idx);
//            }
//        }
//        
//        None
//            
//    }
//    
//    fn column_matches(&self, col_idx: usize, values: &[T], row_indices: &[usize]) -> bool {
//        let new_col_nnz = values.len();
//        let col_start = self.col_ptr[col_idx];
//        let col_end = self.col_ptr[col_idx + 1];
//        let col_nnz = col_end - col_start;
//
//        if col_nnz != new_col_nnz {
//            return false;
//        }
//
//        let mut found = true;
//        for i in 0..col_nnz {
//            if row_indices[i] != self.row_indices[col_start + i] 
//            || values[i] != self.values[col_start + i]
//            {
//                found = false;
//                break; 
//            }
//
//        }
//
//        found 
//    }
//
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

//    #[test]
//    fn test_csc_matrix() {
//         let mut mat: CooMatrix<i64> = CooMatrix::new();
//         mat.insert(0, 0, -1);
//         mat.insert(1, 0, 1);
//         mat.insert(4, 5, 2);
//         mat.insert(0, 0, 3); // duplicate entry
//         mat.insert(3, 2, 4);
//         mat.insert(1, 0, 5); // duplicate entry
//
        
    //     // check sorting
    //     let mat: CscMatrix<i64> = CscMatrix::from(mat);
    //     assert_eq!(mat.values, vec![2, 6, 4, 2]);
    //     assert_eq!(mat.col_ptr, vec![0, 2, 2, 3, 3, 3, 4]);
    //     assert_eq!(mat.row_indices, vec![0, 1, 3, 4]);

    //     // assert_eq!(mat.values[mat.col_slice[0]..mat.row_slice[1]], vec![2]);
    //     // assert_eq!(mat.values[mat.col_slice[2]..mat.row_slice[3]], vec![]);
    //     // assert_eq!(mat.values[mat.row_slice[4]..mat.row_slice[5]], vec![2]);
    // }

}
