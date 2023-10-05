use csc411_image::{Read, GrayImage};
use std::env;
use std::vec::Vec;

use array2::Array2;

fn main() {

    let input = env::args().nth(1);
    //println!("Here");
    let img = GrayImage::read(input.as_deref()).unwrap();

    let mut my_vector: Vec<i32> = Vec::new();

    println!("This img height: {}", img.height);
    println!("This img width: {}", img.width);

    for pixel in img.pixels.iter().take(9) {
        my_vector.push((pixel.value as u32).try_into().unwrap());
    }

    let array = Array2::new(3, 3, my_vector);
    
    array.f_bool_ValidElement(my_vector);

    // for (i, element) in array.get_data().iter().enumerate() {
    //     print!("{}, ", element);
    //     if (i + 1) % 9 == 0{
    //         println!();
    //     }
    // }

    //println!("--- END OF PROGRAM ---");
}