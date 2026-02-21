use super::*;


// COO MATRIX
impl<T: Scalar> From<DokMatrix<T>> for CooMatrix<T> {
    fn from(matrix: DokMatrix<T>) -> Self {
        let (data, nrow, ncol) = matrix.into_raw();
        let mut values = Vec::new();
        let mut row_indices = Vec::new();
        let mut col_indices = Vec::new();
        for ((r, c), val) in data {
            values.push(val);
            row_indices.push(r);
            col_indices.push(c);
        }

        CooMatrix::from_raw(
            values,
            row_indices,
            col_indices,
            nrow,
            ncol)
    }
}

impl<T: Scalar> From<CsrMatrix<T>> for CooMatrix<T> {

    fn from(matrix: CsrMatrix<T>) -> Self {
        let (values, row_ptr, col_indices, nrow, ncol) = matrix.into_raw();
        let nnz = row_ptr[nrow];
        let mut row_indices = Vec::with_capacity(nnz);

        for row in 0..nrow {
            let row_nnz = row_ptr[row+1] - row_ptr[row];
            for _i in 0..row_nnz {
                row_indices.push(row);
            }
        }

        CooMatrix::from_raw_unsafe(
            values,
            row_indices,
            col_indices,
            nrow, ncol,
            Order::RowMajor)
    }

}

impl<T: Scalar> From<CscMatrix<T>> for CooMatrix<T> {

      fn from(matrix: CscMatrix<T>) -> Self {
        let (values, col_ptr, row_indices, nrow, ncol) = matrix.into_raw();
        let nnz = col_ptr[ncol];
        let mut col_indices = Vec::with_capacity(nnz);

        for col in 0..ncol {
            let col_nnz = col_ptr[col+1] - col_ptr[col];
            for _i in 0..col_nnz {
                col_indices.push(col);
            }
        }

        CooMatrix::from_raw_unsafe(
            values,
            row_indices,
            col_indices,
            nrow,
            ncol,
            Order::ColMajor)
        
    }

}

// CSR MATRIX
impl<T: Scalar> From<CooMatrix<T>> for CsrMatrix<T> {

    fn from(mut matrix: CooMatrix<T>) -> Self {
        // Algorithm works only if row-sorted, duplicates removed
        matrix.canonical_format(Order::RowMajor); 
        let nnz = matrix.nnz();
        let (values, row_indices, col_indices, nrow, ncol) = matrix.into_raw();
        
        // count non zero entries per row
        let mut row_counts : Vec<usize> = vec![0;  nrow];
        for row in row_indices {
            row_counts[row] += 1;
        } 
                
        let mut row_ptr: Vec<usize> = Vec::with_capacity(nrow + 1);
        row_ptr.push(0);
        for count in row_counts {
            row_ptr.push(row_ptr.last().unwrap() + count);
        }
        
        assert_eq!(row_ptr.len(), nrow + 1);
        assert_eq!(*row_ptr.last().unwrap(), nnz);
        
        CsrMatrix::from_raw(
            values,
            row_ptr,
            col_indices,
            nrow, 
            ncol
        )
    }

}


// CSC MATRIX
impl<T: Scalar> From<CooMatrix<T>> for CscMatrix<T> {

    fn from(mut matrix: CooMatrix<T>) -> Self {
        
        // Algorithm works only if col-sorted, duplicates removed
        matrix.canonical_format(Order::ColMajor);
        let nnz = matrix.nnz();
        
        let (values, row_indices, col_indices, nrow, ncol) = matrix.into_raw();

        // count non zero entries per col
        let mut col_counts : Vec<usize> = vec![0;  ncol];
        for col in col_indices {
            col_counts[col] += 1;
        } 
                
        let mut col_ptr: Vec<usize> = Vec::with_capacity(ncol + 1);
        col_ptr.push(0);
        for count in col_counts {
            col_ptr.push(col_ptr.last().unwrap() + count);
        }
        
        assert_eq!(col_ptr.len(), ncol + 1);
        assert_eq!(*col_ptr.last().unwrap(), nnz);
        
        CscMatrix::from_raw(
            values,
            col_ptr,
            row_indices,
            nrow,
            ncol)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    // Test data helpers
    fn create_simple_coo_matrix() -> CooMatrix<i64> {
        // 3x4 matrix:
        //   0  1  2  3
        // 0[1  0  2  0]
        // 1[0  0  0  0]
        // 2[0  3  0  4]
        let mut coo : CooMatrix<i64> = CooMatrix::new();
        coo.insert(0, 0, 1);
        coo.insert(0, 2, 2);
        coo.insert(2, 1, 3);
        coo.insert(2, 3, 4);
        coo
    }

//    #[test]
//    fn test_simple_coo_to_csr() {
//        let mut coo = create_simple_coo_matrix();
//        let csr = CsrMatrix::from(coo);
//        assert_eq!(csr.nnz(), 4);
//        assert_eq!(csr.row_ptr, vec![0,2,2,4]);
//        assert_eq!(csr.col_indices, vec![0,2,1,3]);
//    }
//
//    #[test]
//    fn test_simple_coo_to_csc() {
//        let mut coo = create_simple_coo_matrix();
//        let csc = CscMatrix::from(coo);
//        assert_eq!(csc.nnz(), 4);
//        assert_eq!(csc.col_ptr, vec![0,1,2,3,4]);
//        assert_eq!(csc.row_indices, vec![0,2,0,2]);
//    }
//    
    #[test]
    fn coo_to_csr_conversion() {
        let mut coo = create_simple_coo_matrix();
        coo.sort_by_row();   
        let (coo_values, coo_rows, coo_cols, nrow, ncol) = coo.clone().into_raw();
        

        let csr = CsrMatrix::from(coo);
        let mut coo_from_csr = CooMatrix::from(csr);
        let (vs, rs , cs) = coo_from_csr.view_raw();
        
        assert_eq!(vs, coo_values);
        assert_eq!(rs, coo_rows ); 
        assert_eq!(cs, coo_cols );   

        assert_eq!(coo_from_csr.nrow(), nrow);
        assert_eq!(coo_from_csr.ncol(), ncol); 
    }

    #[test]
    fn coo_to_csc_conversion() {
        let mut coo = create_simple_coo_matrix();
        coo.sort_by_col();    
        let (coo_values, coo_rows, coo_cols, nrow, ncol) = coo.clone().into_raw();
        
    
        let csc = CscMatrix::from(coo);
        let mut coo_from_csc = CooMatrix::from(csc);
        let (vs, rs , cs) = coo_from_csc.view_raw();
        
        assert_eq!(vs, coo_values );
        assert_eq!(rs, coo_rows ); 
        assert_eq!(cs, coo_cols );    

        assert_eq!(coo_from_csc.nrow(), nrow);
        assert_eq!(coo_from_csc.ncol(), ncol);
    }

}
