use csc411_image::{Read, GrayImage};
use std::env;
use std::vec::Vec;

use array2::Array2;

fn main() {
    let input = env::args().nth(1);
    let img = GrayImage::read(input.as_deref()).unwrap();

    let mut my_vector: Vec<i32> = Vec::new();

    println!("This img height: {}", img.height);
    println!("This img width: {}", img.width);

    for pixel in &img.pixels {
        my_vector.push(pixel.value.into());
    }

    let mut sudoku_table: Vec<Vec<i32>> = vec![vec![0; img.width as usize]; img.height as usize];

    for row in 0..img.height {
        for col in 0..img.width {
            let index = (row * img.width + col) as usize;
            sudoku_table[row as usize][col as usize] = my_vector[index];
        }
    }

    for row in &sudoku_table {
        for &element in row {
            print!("{:3} ", element);
        }
        println!();
    }
}
    // let array = Array2::new(3, 3, my_vector);
    
    // println!("{}",array.f_bool_valid_element());

    // for (i, element) in array.get_data().iter().enumerate() {
    //     print!("{}, ", element);
    //     if (i + 1) % 9 == 0{
    //         println!();
    //     }
    // }
    //      --- TO HERE ---^^^

    //println!("--- END OF PROGRAM ---");