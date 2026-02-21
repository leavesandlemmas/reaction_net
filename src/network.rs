pub mod builder;

pub type Coef = i32;
pub type Complex = CompressedVector<Coef>;

use crate::sparse::*;
pub use builder::NetworkBuilder;

