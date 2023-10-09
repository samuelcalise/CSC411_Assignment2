use csc411_image::{Read, GrayImage};
use std::env;
use std::vec::Vec;
use array2::Array2;
use std::process;

fn main() {

    //Getting The File Name
    let input = env::args().nth(1);

    let img = match input {
        Some(filename) => {
            match GrayImage::read(Some(filename.as_str())) {
                Ok(img) => img,
                Err(_err) => {
                    //eprintln!("No File Found In Directory"); //Works
                    process::exit(0); // False 
                }
            }
        }
        //Works
        None => {
            //eprintln!("No input filename provided.");
            process::exit(0); //False
        }
    };

    //Works
    if img.height != 9 && img.width != 9{
        process::exit(1);
    }

    let mut my_vector: Vec<i32> = Vec::new();

    for pixel in &img.pixels {
        my_vector.push(pixel.value.into());
    }


    let sudoku_array = Array2::new(9, 9, my_vector);

    // Check if the Sudoku array is a valid Sudoku puzzle
    if sudoku_array.is_valid_sudoku() {
        // println!("The Sudoku is a valid puzzle.");
        // process::exit(1); //True
        //Continue

    } else {
        println!("The Sudoku is not a valid puzzle.");
        process::exit(0); // False
    }

    for row in 0..3 {
        for col in 0..3 {
            let sub_square = sudoku_array.get_subsquare(row * 3, col * 3);
            if !sub_square.is_unique() {
                println!("The 3x3 subsquare at row {} col {} is not valid.", row, col);
                process::exit(0);
            }
        }
    }

    println!("Made it");
    process::exit(1);
}