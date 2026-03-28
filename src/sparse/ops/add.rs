use crate::sparse::format::*;
use std::ops::{AddAssign, Add};

// Implement Add and AddAssign


impl<T: AddAssign> AddAssign for CompressedVector<T> {

    fn add_assign(&mut self, other: Self){


    } 
}

/*

example of implementing AddAssign by reference

#[derive(Clone, Debug)]
struct Point<T> {
    x: T,
    y: T,
} 

impl<T> Point<T> {
    
    pub fn from_raw(x: T, y: T) -> Self {
        Self {x, y}
    }

    pub fn from_tuple(tpl : (T, T)) -> Self {
        Self {x: tpl.0, y: tpl.1}
    }    
   
}

impl<T:AddAssign> AddAssign for Point<T> {
    
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T:AddAssign + Clone> AddAssign<&Point<T>> for Point<T> {
    
    fn add_assign(&mut self, other: &Point<T>) {
        self.x += other.x.clone();
        self.y += other.y.clone();
    }
}

impl<T> AddAssign<&Point<T>> for Point<T>
where
    for<'a> T: AddAssign<&'a T>,
{
    fn add_assign(&mut self, other: &Point<T>) {
        self.x += &other.x;
        self.y += &other.y;
    }
}
*/
