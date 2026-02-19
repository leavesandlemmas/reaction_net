use super::*;

#[derive(Debug, Clone)]
pub struct CsrMatrix<T> {
    values: Vec<T>,
    row_ptr: Vec<usize>,
    col_indices: Vec<usize>,
    nrow: usize,
    ncol: usize,
}

impl<T: Scalar> CsrMatrix<T> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            row_ptr: Vec::new(),
            col_indices: Vec::new(),
            nrow: 0,
            ncol: 0,
        }
    }

    pub fn from_raw(
        values: Vec<T>, 
        row_ptr: Vec<usize>, 
        col_indices: Vec<usize>, 
        nrow : usize, 
        ncol: usize ) -> Self {
            assert_eq!(row_ptr.len(), nrow + 1);
            assert_eq!(values.len(), col_indices.len());
            let min_ncol = col_indices.iter().max().unwrap() + 1;
            assert!(ncol >= min_ncol);
            Self{values, row_ptr, col_indices, nrow, ncol}

    }

    pub fn into_raw(self) ->(Vec<T>, Vec<usize>, Vec<usize>, usize, usize) {
        (self.values, self.row_ptr, self.col_indices, self.nrow, self.ncol)
    }
}

impl<T>  SparseMatrix for CsrMatrix<T>  {
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

    // #[test]
    // fn test_csr_matrix() {
    //     let mut mat: CooMatrix<i64> = CooMatrix::new();
    //     mat.insert(0, 0, -1);
    //     mat.insert(1, 0, 1);
    //     mat.insert(4, 5, 2);
    //     mat.insert(0, 0, 3); // duplicate entry
    //     mat.insert(3, 2, 4);
    //     mat.insert(1, 0, 5); // duplicate entry

    //     // check sorting
    //     let mat: CsrMatrix<i64> = CsrMatrix::from(mat);
    //     assert_eq!(mat.values, vec![2, 6, 4, 2]);
    //     assert_eq!(mat.row_ptr, vec![0, 1, 2, 2, 3, 4]);
    //     assert_eq!(mat.col_indices, vec![0, 0, 2, 5]);

    //     assert_eq!(mat.values[mat.row_ptr[0]..mat.row_ptr[1]], vec![2]);
    //     assert_eq!(mat.values[mat.row_ptr[2]..mat.row_ptr[3]], vec![]);
    //     assert_eq!(mat.values[mat.row_ptr[4]..mat.row_ptr[5]], vec![2]);
    // }
}
