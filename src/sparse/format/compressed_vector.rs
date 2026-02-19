
use super::*;


#[derive(Debug, Clone)]
pub struct CompressedVector<T> {
    values: Vec<T>,
    indices: Vec<usize>,
    dim: usize,
    canonical: bool,
}

impl<T: Scalar> CompressedVector<T> {
    
    fn new(dim : usize ) -> Self {
        Self {
            values: Vec::new(),
            indices: Vec::new(),
            dim,
            canonical: false,
        }
    }

    fn insert(&mut self, index: usize, value: T) {
        self.values.push(value);
        self.indices.push(index);
        self.dim = self.dim.max(index + 1);
        self.canonical = false;
    }

    fn canonical_format(&mut self) {
        self.sum_duplicates();
    }

    fn sum_duplicates(&mut self) {
        if self.canonical {
            return ();
        }
        self.sort();
        let mut read: usize = 1;
        let mut write: usize = 0;
        while read < self.nnz() {
            // if indices are the same, then duplicate
            if self.indices[write] == self.indices[read]
            {
                let val = self.values[read].clone();
                self.values[write] += val;
            } else {
                // next position
                write += 1;
                // overwrite duplicate values
                if write != read {
                    self.values[write] = self.values[read].clone();
                    self.indices[write] = self.indices[read];
                }
            }
            read += 1;
        }

        // write pointer is the last valid position; discard remaining data
        self.values.truncate(write + 1);
        self.indices.truncate(write + 1);
        self.canonical = true;

    }

    pub fn sort(&mut self) {
        let mut p: Vec<usize> = (0..self.nnz()).collect();

        p.sort_by(|&x, &y| self.indices[x]
                .cmp(&self.indices[y])
            );

        self.permute(&mut p);
    }

    fn permute(&mut self, p : &mut [usize]) {
        let nnz = p.len();
        assert!(nnz == self.nnz());
        // apply permutation by cyclic decomposition
        for i in 0..nnz {
            if p[i] != i {
                // skip fixed points
                let mut current = i;

                let temp_val = self.values[i].clone();
                let temp_idx = self.indices[i];
               
                loop {
                    // inverse transposition
                    let next = p[current];
                    p[current] = current;

                    if next == i {
                        self.values[current] = temp_val;
                        self.indices[current] = temp_idx;
                        break;
                    }

                    self.values[current] = self.values[next].clone();
                    self.indices[current] = self.indices[next];
                    current = next;
                }
            }
        }
    }

    pub fn nnz(&self) -> usize {
        self.values.len()
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compressed_vec() {
        let mut vec : CompressedVector<i64> = CompressedVector::new(4);
        vec.insert(2, 1);
        vec.insert(3, 1);
        vec.insert(2, 1);
        
        vec.canonical_format();
        assert_eq!(vec.values, vec![2, 1]);
        assert_eq!(vec.indices, vec![2, 3]);

    }
}
