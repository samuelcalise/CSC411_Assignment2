use csc411_image::{Read, GrayImage};
use std::env;
use std::vec::Vec;

use array2::Array2;

fn main() {

    let input = env::args().nth(1);
    //println!("Here");
    let img = GrayImage::read(input.as_deref()).unwrap();

    let mut my_vector: Vec<i32> = Vec::new();

    for pixel in img.pixels.iter().take(9) {
        my_vector.push((pixel.value as u32).try_into().unwrap());
    }

    let array = Array2::new(3, 3, my_vector);

    for element in array.get_data() {
        println!("{}", element);
    }

    //println!("--- END OF PROGRAM ---");
}