use csc411_image::{Read, GrayImage};
use core::ops::RangeBounds;

pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Clone + RangeBounds<T>> Array2<T> {
    /// Creates a new `Array2`.
    ///
    /// # Arguments
    ///
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`.
    pub fn new(row: usize, column: usize, data: Vec<T>) -> Self {
        Array2 {
            width: column,
            height: row,
            data,
        }
    }

    /*------------------------\
    |    Public Functions     |
    \------------------------*/

    pub fn f_bool_ValidElement(&self) {
        let mut unique_vector::<T> = vec![];
        for element in self.get_data(){
                if element.contains(unique_vector){
                    unique_vector.push(element)
                }
                else{
                    continue;
                }
            }
        
    }

    pub fn f_bool_ValidRow() {
        println!("In public function valid row");
    }

    pub fn f_bool_ValidCol() {
        println!("In public function valid col");
    }

    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn f_bool_EmptyImage() {
        println!("In public function empty img");
    }

    pub fn f_bool_ValidSquare() {
        println!("In public function empty img");
    }
}