use csc411_image::{Read, GrayImage};
use std::env;
use std::vec::Vec;
use array2::Array2;
use std::process;

fn main() {

    //Getting The File Name
    let input = env::args().nth(1);

    
    /*
        Verifying the input via terminal command line whether the 
        filename is a valid GrayImage, and whether the file name 
        is provided within the teminal cargo run command
    */
    let img = match input {
        Some(filename) => {
            match GrayImage::read(Some(filename.as_str())) {
                Ok(img) => img,
                Err(_err) => {
                    //eprintln!("No File Found In Directory"); //Works
                    process::exit(1); // Failed
                }
            }
        }
        //Works
        None => {
            //eprintln!("No input filename provided.");
            process::exit(1); // Failed
        }
    };

    //initial vector
    let mut my_vector: Vec<i32> = Vec::new();

    //getting all pixel values and inserting into init vector
    for pixel in &img.pixels {
        my_vector.push(pixel.value.into());
    }


    //Works -- check for most basic board/puzzle checks of pgm image
    if img.height != 9 && img.width != 9 && my_vector.len() != 81{
        process::exit(1); // Failed
    }

    //creating array2 object from lib.rs (invariant)
    let sudoku_array = Array2::new(9, 9, my_vector);

    /*
        Verifying the iter_row_major and iter_col_major check via the function
        is_valid_sudoku() which return true or false based on the developed 
        row and column iterators in lib.rs
    */
    if sudoku_array.is_valid_sudoku() {
        //valid continue to next step

    } else {
        //println!("Sudoku is not valid");
        process::exit(1); // Failed
    }


    /*
        Verifying The 3x3 subsquares of the 9x9 sudoku board/puzzle
    */
    for row in 0..3 {
        for col in 0..3 {
            //making a new subsqaure within array2 object at every iteration
            let sub_square = sudoku_array.get_subsquare(row * 3, col * 3);
            //calling a helper function to determine if the elements within the
            //3x3 sqaure are NOT unique, if this is the case -  Then the code's run
            //time would end due to not being a valid sudoku board/puzzle
            if !sub_square.is_unique() {
                //println!("!!Error!! => The 3x3 subsquare at row {} col {} is not valid.", row, col);
                process::exit(1);// Failed
            }
        }
    }

    //println!("Made it");
    process::exit(0);// Passed
}