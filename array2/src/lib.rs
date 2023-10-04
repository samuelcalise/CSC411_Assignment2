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
    |    Public Functions    |
    \------------------------*/

    pub fn f_bool_ValidElement(& mut self) {
        println!("In public function valid element");
    }

    pub fn f_bool_ValidRow() {
        println!("In public function valid row");
    }

    pub fn f_bool_ValidCol() {
        println!("In public function valid col");
    }

    pub fn f_bool_EmptyImage() {
        println!("In public function empty img");
    }

    pub fn f_bool_ValidSquare() {
        println!("In public function empty img");
    }
}