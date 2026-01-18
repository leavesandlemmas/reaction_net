use std::collections::HashMap;
use std::ops::AddAssign;
// sparse matrix library

// dictionary of keys
// + element access
pub struct DokMatrix<T> {
    data: HashMap<(usize, usize), T>,
    dim: Dim,
}

impl<T: AddAssign + Clone> DokMatrix<T> {
    pub fn new() -> Self {
        let data = HashMap::new();
        let dim = Dim::new();
        Self { data, dim }
    }

    pub fn insert(&mut self, row: usize, col: usize, value: T) {
        let key = (row, col);
        self.dim.update(row, col);
        self.data
            .entry(key)
            .and_modify(|e| *e += value.clone())
            .or_insert(value);
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        let key = (row, col);
        self.data.get(&key)
    }

    pub fn nnz(&self) -> usize {
        self.data.len()
    }
}

// Coordinate format
// + supports duplicate entries
// + no single element access
pub struct CooMatrix<T> {
    values: Vec<T>,
    row_index: Vec<usize>,
    col_index: Vec<usize>,
    dim: Dim,
    canonical: bool,
    sorted: bool,
}

impl<T: AddAssign + Clone> CooMatrix<T> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            row_index: Vec::new(),
            col_index: Vec::new(),
            dim: Dim::new(),
            canonical: false,
            sorted: false,
        }
    }

    pub fn insert(&mut self, row: usize, col: usize, value: T) {
        self.dim.update(row, col);
        self.values.push(value);
        self.row_index.push(row);
        self.col_index.push(col);
        self.canonical = false;
        self.sorted = false;
    }

    pub fn is_canonical_format(&self) -> bool {
        self.canonical
    }

    pub fn is_sorted(&self) -> bool {
        self.sorted
    }

    pub fn canonical_format(&mut self) {
        // automatically canonical
        if self.nnz() < 2 {
            self.canonical = true;
            return ();
        }

        // duplicates are adjacent after sorting
        if !self.sorted {
            self.sort_data();
        }

        let mut read: usize = 1;
        let mut write: usize = 0;
        while read < self.nnz() {
            // if indices are the same, then duplicate
            if self.row_index[write] == self.row_index[read]
                && self.col_index[write] == self.col_index[read]
            {
                let val = self.values[read].clone();
                self.values[write] += val;
            } else {
                // next position
                write += 1;
                // overwrite duplicate values
                if write != read {
                    self.values[write] = self.values[read].clone();
                    self.row_index[write] = self.row_index[read];
                    self.col_index[write] = self.col_index[read];
                }
            }
            read += 1;
        }

        // write pointer is the last valid position; discard remaining data
        self.values.truncate(write + 1);
        self.row_index.truncate(write + 1);
        self.col_index.truncate(write + 1);

        self.canonical = true;
    }

    pub fn sort_data(&mut self) {
        // sort arrays using permutation vector
        let mut p: Vec<usize> = (0..self.values.len()).collect();
        p.sort_by(|&x, &y| {
            self.row_index[x]
                .cmp(&self.row_index[y])
                .then_with(|| self.col_index[x].cmp(&self.col_index[y]))
        });

        // apply permutation by cyclic decomposition
        for i in 0..p.len() {
            if p[i] != i {
                // skip fixed points
                let mut current = i;

                let temp_val = self.values[i].clone();
                let temp_row = self.row_index[i];
                let temp_col = self.col_index[i];

                loop {
                    // inverse transposition
                    let next = p[current];
                    p[current] = current;

                    if next == i {
                        self.values[current] = temp_val;
                        self.row_index[current] = temp_row;
                        self.col_index[current] = temp_col;
                        break;
                    }

                    self.values[current] = self.values[next].clone();
                    self.row_index[current] = self.row_index[next];
                    self.col_index[current] = self.col_index[next];

                    current = next;
                }
            }
        }

        self.sorted = true
    }

    pub fn nnz(&self) -> usize {
        self.values.len()
    }
}

pub struct CsrMatrix<T> {
    values: Vec<T>,
    row_slice: Vec<usize>,
    col_index: Vec<usize>,
    dim: Dim,
}

impl<T: AddAssign + Clone> CsrMatrix<T> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            row_slice: Vec::new(),
            col_index: Vec::new(),
            dim : Dim::new(),
        }
    }
    // pub fn add_row(&mut self, col : usize, value :  Vec<usize> )
}

impl<T: AddAssign + Clone> From<CooMatrix<T>> for CsrMatrix<T> {
    fn from(mut matrix: CooMatrix<T>) -> Self {
        matrix.canonical_format();
        let mut row_slice: Vec<usize> = Vec::with_capacity(matrix.dim.row() + 1);

        let mut row: usize = 0;
        row_slice.push(row);
        for k in 0..matrix.nnz() {
            while row != matrix.row_index[k] {
                row_slice.push(k);
                row += 1;
            }
        }

        row_slice.push(matrix.nnz());

        Self {
            values: matrix.values,
            row_slice,
            col_index: matrix.col_index,
            dim: matrix.dim,
        }
    }
}

pub struct CscMatrix<T> {
    values: Vec<T>,
    col_slice: Vec<usize>,
    row_index: Vec<usize>,
    dim: Dim,
}

impl<T: AddAssign + Clone> CscMatrix<T> {
    pub fn new() -> Self{
        Self {
            values: Vec::new(),
            col_slice: Vec::new(),
            row_index: Vec::new(),
            dim : Dim::new(),
        }
    }
}

impl<T: AddAssign + Clone> From<CooMatrix<T>> for CscMatrix<T> {
    fn from(mut matrix: CooMatrix<T>) -> Self {
        matrix.canonical_format();
        let mut col_slice: Vec<usize> = Vec::with_capacity(matrix.dim.col() + 1);

        let mut col: usize = 0;
        col_slice.push(col);
        for k in 0..matrix.nnz() {
            while col != matrix.col_index[k] {
                col_slice.push(k);
                col += 1;
            }
        }

        col_slice.push(matrix.nnz());

        Self {
            values: matrix.values,
            col_slice,
            row_index: matrix.row_index,
            dim: matrix.dim,
        }
    }

}

struct Dim {
    row: usize,
    col: usize,
}

impl Dim {
    fn new() -> Self {
        Self { row: 0, col: 0 }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn update(&mut self, row: usize, col: usize) {
        self.row = self.row.max(row + 1);
        self.col = self.col.max(col + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dok_matrix() {
        let mut mat: DokMatrix<i64> = DokMatrix::new();
        assert_eq!(mat.dim.row(), 0);
        assert_eq!(mat.dim.col(), 0);
        mat.insert(0, 0, 1);

        assert_eq!(mat.dim.row(), 1);
        assert_eq!(mat.dim.col(), 1);

        mat.insert(0, 1, -1);
        assert_eq!(mat.dim.row(), 1);
        assert_eq!(mat.dim.col(), 2);
        mat.insert(0, 0, 1); // should be two
        let val = mat.get(0, 0);
        assert!(val.is_some());
        assert_eq!(val.unwrap().clone(), 2);

        mat.insert(7, 5, 5);
        assert_eq!(mat.dim.row(), 8);
        assert_eq!(mat.dim.col(), 6);
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
        mat.sort_data();
        assert_eq!(mat.values, vec![-1, 3, 1, 5, 4, 2]);
        assert_eq!(mat.row_index, vec![0, 0, 1, 1, 3, 4]);
        assert_eq!(mat.col_index, vec![0, 0, 0, 0, 2, 5]);

        // check canonical format
        mat.canonical_format();
        assert_eq!(mat.values, vec![2, 6, 4, 2]);
        assert_eq!(mat.row_index, vec![0, 1, 3, 4]);
        assert_eq!(mat.col_index, vec![0, 0, 2, 5]);
    }

    #[test]
    fn test_csr_matrix() {
        let mut mat: CooMatrix<i64> = CooMatrix::new();
        mat.insert(0, 0, -1);
        mat.insert(1, 0, 1);
        mat.insert(4, 5, 2);
        mat.insert(0, 0, 3); // duplicate entry
        mat.insert(3, 2, 4);
        mat.insert(1, 0, 5); // duplicate entry

        // check sorting
        let mat: CsrMatrix<i64> = mat.into();
        assert_eq!(mat.values, vec![2, 6, 4, 2]);
        assert_eq!(mat.row_slice, vec![0, 1, 2, 2, 3, 4]);
        assert_eq!(mat.col_index, vec![0, 0, 2, 5]);

        assert_eq!(mat.values[mat.row_slice[0]..mat.row_slice[1]], vec![2]);
        assert_eq!(mat.values[mat.row_slice[2]..mat.row_slice[3]], vec![]);
        assert_eq!(mat.values[mat.row_slice[4]..mat.row_slice[5]], vec![2]);
    }

    #[test]
    fn test_csc_matrix() {
        let mut mat: CooMatrix<i64> = CooMatrix::new();
        mat.insert(0, 0, -1);
        mat.insert(1, 0, 1);
        mat.insert(4, 5, 2);
        mat.insert(0, 0, 3); // duplicate entry
        mat.insert(3, 2, 4);
        mat.insert(1, 0, 5); // duplicate entry

        // check sorting
        let mat: CscMatrix<i64> = mat.into();
        assert_eq!(mat.values, vec![2, 6, 4, 2]);
        assert_eq!(mat.col_slice, vec![0, 2, 2, 3, 4]);
        assert_eq!(mat.row_index, vec![0, 1, 3, 4]);

        // assert_eq!(mat.values[mat.col_slice[0]..mat.row_slice[1]], vec![2]);
        // assert_eq!(mat.values[mat.col_slice[2]..mat.row_slice[3]], vec![]);
        // assert_eq!(mat.values[mat.row_slice[4]..mat.row_slice[5]], vec![2]);
    }
}
