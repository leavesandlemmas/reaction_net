pub mod dok;
pub mod coo;
pub mod csr;
pub mod csc;

use dok::DokMatrix;
use coo::CooMatrix;
use csr::CsrMatrix;
use csc::CscMatrix;

pub trait SparseMatrix {
    fn nrow(&self) -> usize;
    fn ncol(&self) -> usize;
    fn nnz(&self) -> usize;
    fn shape(&self) -> (usize, usize) {
        (self.nrow(), self.ncol())
    }

    //fn get(&self, row: usize, col: usize) -> Option<&T>;
}

pub trait Scalar:
    Clone
    + std::ops::AddAssign 
    + PartialEq { }

impl<T: Clone + std::ops::AddAssign + PartialEq> Scalar for T {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Order {
    RowMajor,
    ColMajor,
}


// pub trait RowAccessible<T>: SparseMatrix<T> {
//     type RowIter<'a> : Iterator<Item = (usize, &'a T)> where Self: 'a;
//     fn row(&self, i: usize) -> Self::RowIter<'_>; 
// }
// Marker trait for formats that support efficient column iteration
// pub trait ColAccessible<T>: SparseMatrix<T> {
//     type ColIter<'a>: Iterator<Item = (usize, &'a T)> where Self: 'a;
//     fn col(&self, j: usize) -> Self::ColIter<'_>;
// }

// Mutable sparse matrix interface
// pub trait SparseMatrixMut<T>: SparseMatrix<T> {
//     fn set(&mut self, row: usize, col: usize, value: T);
//     fn remove(&mut self, row: usize, col: usize) -> Option<T>;
// }



