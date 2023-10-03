


pub use array2::Array2;

fn main() {
    let array = Array2::new(3, 3, 0); // Example usage of Array2::new
    let something = Array2::f_bool_ValidElement();      // Call the public function
    println!("{:?}", something);
}