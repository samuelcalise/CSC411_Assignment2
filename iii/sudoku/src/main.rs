use csc411_image::{Read, GrayImage};
use std::env;
use std::vec::Vec;
use array2::Array2;
use std::process;
use std::io;

fn main(){

    //Getting The File Name
    let mut input = String::new();
    let command_arguments: Vec<String> = env::args().collect();

    if command_arguments.len() != 2 {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_err) => {
                process::exit(1);
            }
        }
    } else {
        let filename = &command_arguments[1];
        input = filename.to_string();
    }
    
    /*
        Verifying the input via terminal command line whether the 
        filename is a valid GrayImage, and whether the file name 
        is provided within the teminal cargo run command
    */
    let img = match GrayImage::read(Some(input.trim())) {
        Ok(img) => img,
        Err(_) => {
            let path_str = input.trim();
            let path_str_ref: &str = &path_str;
            match GrayImage::read(Some(path_str_ref)) {
                Ok(img) => img,
                Err(_) => {
                    eprintln!("Invalid input or file not found.");
                    process::exit(1);
                }
            }
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