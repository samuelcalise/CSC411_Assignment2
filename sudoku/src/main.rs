


pub use array2::Array2;

fn main() {
    let mut array = Array2::new(3, 3, 0); // Example usage of Array2::new

    //let something = array.f_bool_ValidElement();      // Call the public function
    array.f_bool_ValidElement();
    //touches library function and works :)
}