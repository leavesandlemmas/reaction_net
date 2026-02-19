use super::*;

// Coordinate format
// + supports duplicate entries
// + no single element access
pub struct CooMatrix<T> {
    values: Vec<T>,
    row_indices: Vec<usize>,
    col_indices: Vec<usize>,
    nrow: usize, 
    ncol: usize,
    canonical: Option<Order>,
    sorted: Option<Order>,
}

impl<T: Scalar> CooMatrix<T> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            row_indices: Vec::new(),
            col_indices: Vec::new(),
            nrow:0, 
            ncol: 0,
            canonical: None,
            sorted: None,
        }
    }

    pub fn from_raw(values: Vec<T>, row_indices: Vec<usize>, col_indices: Vec<usize>, 
        nrow: usize, ncol: usize) -> Self {
        Self{values, row_indices, col_indices, nrow, ncol, canonical: None, sorted: None}
    }

    pub(super) fn from_raw_ordered(values: Vec<T>, row_indices: Vec<usize>, col_indices: Vec<usize>, 
        nrow: usize, ncol: usize, ord : Order) -> Self {
        Self{values, row_indices, col_indices, nrow, ncol, canonical: Some(ord), sorted: Some(ord)}
    }

    pub fn into_raw(self) -> ( Vec<T>, Vec<usize>, Vec<usize>, usize, usize) {
        (self.values, self.row_indices, self.col_indices, self.nrow, self.ncol)
    }
    
    pub fn is_canonical_format(&self) -> bool {
        self.canonical.is_some()
    }
    
    pub fn is_sorted(&self) -> bool {
        self.sorted.is_some()
    }

    pub fn insert(&mut self, row: usize, col: usize, value: T) {
        self.nrow = self.nrow.max(row + 1); 
        self.ncol = self.ncol.max(col + 1);
        self.values.push(value);
        self.row_indices.push(row);
        self.col_indices.push(col);
        self.canonical = None;
        self.sorted = None;
    }

    pub fn canonical_format(&mut self, order: Order) {
        // automatically canonical
        if self.nnz() < 2 {
            self.canonical.replace(order);
            return ();
        }

        // duplicates are adjacent after sorting
        match order {
            Order::RowMajor => self.sort_by_row(),
            Order::ColMajor => self.sort_by_col(),
        }
                
        self.sum_duplicates();
        self.canonical.replace(order);
    }

    fn sum_duplicates(&mut self) {
        if self.sorted.is_none() {
            self.sort_by_row();
        }

        let mut read: usize = 1;
        let mut write: usize = 0;
        while read < self.nnz() {
            // if indices are the same, then duplicate
            if self.row_indices[write] == self.row_indices[read]
                && self.col_indices[write] == self.col_indices[read]
            {
                let val = self.values[read].clone();
                self.values[write] += val;
            } else {
                // next position
                write += 1;
                // overwrite duplicate values
                if write != read {
                    self.values[write] = self.values[read].clone();
                    self.row_indices[write] = self.row_indices[read];
                    self.col_indices[write] = self.col_indices[read];
                }
            }
            read += 1;
        }

        // write pointer is the last valid position; discard remaining data
        self.values.truncate(write + 1);
        self.row_indices.truncate(write + 1);
        self.col_indices.truncate(write + 1);

    }

    pub fn sort_by_row(&mut self) {
        let mut p: Vec<usize> = (0..self.nnz()).collect();

        p.sort_by(|&x, &y| self.row_indices[x]
                .cmp(&self.row_indices[y])
                .then_with(|| self.col_indices[x].cmp(&self.col_indices[y])));

        self.permute(&mut p);
        self.sorted.replace(Order::RowMajor);
    }

    pub fn sort_by_col(&mut self) {
        let mut p: Vec<usize> = (0..self.nnz()).collect();

        p.sort_by(|&x , &y| self.col_indices[x]
                .cmp(&self.col_indices[y])
                .then_with(|| self.row_indices[x].cmp(&self.row_indices[y]))) ;

        self.permute(&mut p);
        self.sorted.replace(Order::ColMajor);
    }


    fn permute(&mut self, p : &mut [usize]) {
        assert!(p.len() == self.nnz());
        // apply permutation by cyclic decomposition
        for i in 0..p.len() {
            if p[i] != i {
                // skip fixed points
                let mut current = i;

                let temp_val = self.values[i].clone();
                let temp_row = self.row_indices[i];
                let temp_col = self.col_indices[i];

                loop {
                    // inverse transposition
                    let next = p[current];
                    p[current] = current;

                    if next == i {
                        self.values[current] = temp_val;
                        self.row_indices[current] = temp_row;
                        self.col_indices[current] = temp_col;
                        break;
                    }

                    self.values[current] = self.values[next].clone();
                    self.row_indices[current] = self.row_indices[next];
                    self.col_indices[current] = self.col_indices[next];

                    current = next;
                }
            }
        }
    }
}

impl<T>  SparseMatrix for CooMatrix<T>  {
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
    
    #[test]
    fn test_coo_dim() {
        let coo = create_simple_coo_matrix();
        
        assert_eq!(coo.nnz(), 4);
        assert_eq!(coo.nrow(), 3);
        assert_eq!(coo.nrow(), 3);
    }

    #[test]
    fn test_simple_coo() {
        let mut coo = create_simple_coo_matrix();
        coo.canonical_format(Order::RowMajor);
        assert_eq!(coo.nnz(), 4);
        assert_eq!(coo.row_indices, vec![0,0,2,2]);
        assert_eq!(coo.col_indices, vec![0,2,1,3]);
    }

    #[test]
    fn test_duplicate_entries_summed() {
        let mut coo = create_simple_coo_matrix();
        coo.insert(0, 0, 2);
        coo.insert(0, 0, 3);
        
        coo.sum_duplicates();
        assert_eq!(coo.values[0], 6);
        assert_eq!(coo.nnz(), 4); // Should be the same
    }

    #[test]
    fn test_coo_matrix() {
        let mut mat: CooMatrix<i64> = CooMatrix::new();
        mat.insert(0, 0, -1);
        mat.insert(1, 0, 1);
        mat.insert(4, 5, 2);
        mat.insert(0, 0, 3); // duplicate entry
        mat.insert(3, 2, 4);
        mat.insert(1, 0, 5); // duplicate entry

        // check sorting
        mat.sort_by_row();
        assert_eq!(mat.values, vec![-1, 3, 1, 5, 4, 2]);
        assert_eq!(mat.row_indices, vec![0, 0, 1, 1, 3, 4]);
        assert_eq!(mat.col_indices, vec![0, 0, 0, 0, 2, 5]);

        // check canonical format
        mat.canonical_format(Order::RowMajor);
        assert_eq!(mat.values, vec![2, 6, 4, 2]);
        assert_eq!(mat.row_indices, vec![0, 1, 3, 4]);
        assert_eq!(mat.col_indices, vec![0, 0, 2, 5]);

    }

    #[test]
    fn test_coo_sort() {
        let mut coo: CooMatrix<i64> = CooMatrix::new();
        // A + B -> C + D
        // 2 A + B -> E + F
        coo.insert(5, 3, 1);    
        coo.insert(0, 0, 1);
        coo.insert(3, 1, 1);
        coo.insert(1, 0, 1);
        coo.insert(0, 2, 2);
        coo.insert(1, 2, 1);
        coo.insert(4, 3, 1);
        coo.insert(2, 1, 1);
        
        coo.sort_by_col();
        assert_eq!(coo.row_indices,  vec![0, 1, 2, 3, 0, 1, 4, 5]);
        assert_eq!(coo.col_indices,  vec![0, 0, 1, 1, 2, 2, 3, 3]);

        coo.sort_by_row();
        assert_eq!(coo.row_indices,  vec![0, 0, 1, 1, 2, 3, 4, 5]);
        assert_eq!(coo.col_indices,  vec![0, 2, 0, 2, 1, 1, 3, 3]);

    }


}