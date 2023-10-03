pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Clone> Array2<T> {
    /// Creates a new `Array2`.
    ///
    /// # Arguments
    ///
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`.
    pub fn new(row: usize, column: usize, value: T) -> Self {
        Array2 {
            width: column,
            height: row,
            data: vec![value; row * column]
        }
    }

    /*------------------------\
    |    Private Functions    |
    \------------------------*/

    fn f_bool_ValidElement() {
        println!("In private function valid element");
    }

    fn f_bool_ValidRow() {
        println!("In private function valid row");
    }

    fn f_bool_ValidCol() {
        println!("In private function valid col");
    }

    fn f_bool_EmptyImage() {
        println!("In private function empty img");
    }

    fn f_bool_ValidSquare() {
        println!("In private function empty img");
    }
}