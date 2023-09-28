/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}*/

#[derive(Debug, Clone, Eq, PartialEq)]





impl<T: Clone> Array2<T> {
    /// Creates a new `Array2`.
    ///
    /// # Arguments
    ///
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`.

    /*------------------------*/
    |    Public Functions      |
    /*------------------------*/

    pub mod library_PublicFunctions{
        pub fn f_constructor_BlankSlate()
        {
            println!("In blank slate constructor");
        }
                                //int p_element: usize
        pub fn f_add_ArrayElement()
        {
            println!("In the added element function");
        }
                                //int p_row: usize, int p_column: usize
        pub fn f_find_ValueAtCoord()
        {
            println!("In the find value at coordinate function");
        }

        pub fn f_traverse_IterRowMajor()
        {
            println!("In from row major");
        }

        pub fn f_traverse_IterColMajor()
        {
            println!("In iter col major");
        }
    }
}




/*
    /*------------------------*/
    |    Private Functions     |
    /*------------------------*/

    mod library_PrivateFunctions{

        pub fn f_bool_ValidElement()
        {
            println!("In private function valid element");
        }

        pub fn f_bool_ValidRow()
        {
            println!("In private function valid row");
        }

        pub fn f_bool_ValidCol()
        {
            println!("In private function valid col");
        }

        pub fn f_bool_EmptyImage()
        {
            println!("In private function empty img");
        }

        pub fn f_bool_ValidSquare()
        {
            println!("In private function empty img");
        }

    }
*/
