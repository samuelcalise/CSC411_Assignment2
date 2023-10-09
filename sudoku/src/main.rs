use csc411_image::{Read, GrayImage};
use std::env;
use std::vec::Vec;
use array2::Array2;
use std::process;


fn main() {

    //Getting The File Name
    let input = env::args().nth(1);
    //println!("{:?}", input); Value is "None" if there is nothing within terminal run command

    //let img = GrayImage::read(input.as_deref()).unwrap();

    let img = match input {
        Some(filename) => {
            match GrayImage::read(Some(filename.as_str())) {
                Ok(img) => img,
                Err(_err) => {
                    eprintln!("No File Found In Directory"); //Works
                    process::exit(1);
                }
            }
        }
        //Works
        None => {
            eprintln!("No input filename provided.");
            process::exit(1);
        }
    };

    //Works
    if img.height != 9 && img.width != 9{
        //println!("Bad Board");
        process::exit(1);
    }

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
            print!("{:?} ", element);
        }
        println!();
    }

    println!();
    println!("------Below: Verifying Valid Rows----------");

    for row in &sudoku_table {
        let array = Array2::new(img.width, img.height, row.to_vec());
        println!("{:?}",array.iter_row_major());
    }

    println!();
    println!("-----Below: Verifying Valid Rows------");

    let mut sudoku_table_of_columns: Vec<Vec<i32>> = vec![vec![0; img.width as usize]; img.height as usize];

    for row in 0..9{
        for column in 0..9{
            sudoku_table_of_columns[column][row] = my_vector[row * 9 + column];
        }
    }

    for column in &sudoku_table_of_columns {
        println!("{:?}", column);
    }

    for every_column in &sudoku_table {
        let array = Array2::new(img.width, img.height, every_column.to_vec());
        println!("{:?}",array.iter_col_major());
    }

    println!();
    println!("-----Below: Verifying SubSquares------");

    let mut sudoku_table_of_subsquares: Vec<Vec<Vec<u32>>> = vec![vec![Vec::new(); 3]; 3];

    for sub_square_row in 0..3 {
        for sub_square_col in 0..3 {
            let mut sub_square: Vec<u32> = Vec::new();
            for x_coord in 0..3 {
                for y_coord in 0..3 {
                    let row_idx = sub_square_row * 3 + x_coord;
                    let col_idx = sub_square_col * 3 + y_coord;
                    let value = my_vector[row_idx * 9 + col_idx].try_into().unwrap();
                    sub_square.push(value);
                }
            }
            sudoku_table_of_subsquares[sub_square_row][sub_square_col] = sub_square;
        }
    }

    for row in &sudoku_table_of_subsquares {
        for square in row {
            println!("{:?}", square);
        }
    }

    for row in &sudoku_table_of_subsquares {
        for square in row {
            let array = Array2::new(img.width, img.height, square.to_vec());
            println!("{:?}",array.iter_subsquare_major());
        }
    }

}