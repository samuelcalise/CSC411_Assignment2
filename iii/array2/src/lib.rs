use std::collections::HashSet;

/*
    /------------------------\
    |   Library Structures   |
    \------------------------/
*/

pub struct Array2<T: Clone> {
    width: u32,
    height: u32,
    //invariant
    data: Vec<T>,
}

/*
    The lifetime structure iterators borrows the "data" array from the vector in Array2
    which within every loop of our main.rs we are borrowing without claiming ownership at every
    "lifetime" of every loops iteration. Once an iteration is completed, a new lifetime of the
    board's row and col are created to verify the entire is_valid_sudoku proccess
*/
pub struct RowMajorIterator<'a, T: Clone> {
    array: &'a Array2<T>,
    row: u32,
    col: u32,
}

pub struct ColMajorIterator<'a, T: Clone> {
    array: &'a Array2<T>,
    row: u32,
    col: u32,
}


/*
    When making this iterator for iter_row_major, this implementation
    for this assingment function has the parameter of a generic lifetime
    and generic object T. These traits represent the iterator of iter_row_major
    at every iteration check which is equivalent to one lifetime when checking
    each row. Then once the iterator is finished its iterations of the 1d vector,
    it then moves to the next row which return some Option<T> element that is the
    location of the next iteration step(new lifetime) when verifying the elements
    at each row.
*/
impl<'a, T: Clone> Iterator for RowMajorIterator<'a, T> {
    type Item = Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.array.height {
            let index = (self.row * self.array.width + self.col) as usize;
            let element = self.array.data.get(index).cloned();
            self.col += 1;
            if self.col == self.array.width {
                self.row += 1;
                self.col = 0;
            }
            Some(element)
        } else {
            None
        }
    }
}

/*
    When making this iterator for iter_col_major, this implementation
    for this assingment function has the parameter of a generic lifetime
    and generic object T. These traits represent the iterator of iter_col_major
    at every iteration check which is equivalent to one lifetime when checking
    each column. Then once the iterator is finished its iterations of the 1d vector,
    it then moves to the next column which return some Option<T> element that is the
    location of the next iteration step(new lifetime) when verifying the elements
    at each column.
*/
impl<'a, T: Clone> Iterator for ColMajorIterator<'a, T> {
    type Item = Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < self.array.width {
            let index = (self.col * self.array.height + self.row) as usize;
            let element = self.array.data.get(index).cloned();
            self.row += 1;
            if self.row == self.array.height {
                self.col += 1;
                self.row = 0;
            }
            Some(element)
        } else {
            None
        }
    }
}

/*
    In the Array2 iumplementation, the parameters of Hash,Eq, PartialOrd<i32>, and Display where all intended libraries
    that are used to solve the assignment. Hashmaps were used to populate a reocurring new hashmap that is a collection of
    all values in rows, columns, and 3x3 subsqaure of the sudoku board/puzzle. Comparing Eq and PartialOrd<i32> where used to
    compare an element based on the given hashmap at some iteration of the below function and double checking whether the element
    is between 1..9 without any duplicates. Display was used to print error println during the debugging process of the code.
*/
impl<T: Clone + std::hash::Hash + std::cmp::Eq + std::cmp::PartialOrd<i32> + std::fmt::Display> Array2<T> {
    
    //New function for creating an Array2 object
    pub fn new(row: u32, column: u32, data: Vec<T>) -> Self {
        Array2 {
            width: column,
            height: row,
            data,
        }
    }

    //iter_row_major return the structure RowMajorIterator that obtains the data
    //within the given data of the sudoku board/puzzle
    pub fn iter_row_major(&self) -> RowMajorIterator<T> {
        RowMajorIterator {
            array: self,
            row: 0,
            col: 0,
        }
    }

    //iter_col_major return the structure ColMajorIterator that obtains the data
    //within the given data of the sudoku board/puzzle
    pub fn iter_col_major(&self) -> ColMajorIterator<T> {
        ColMajorIterator {
            array: self,
            row: 0,
            col: 0,
        }
    }

    //Verification function to check all values through the 3x3 are unique
    pub fn iter_subsquare_major(&self) -> bool {
        let mut unique_set = HashSet::new();

        for element in &self.data {
            if !unique_set.insert(element) {
                return false;
            }
        }

        true
    }

    //getter function to recieve the data of the 3x3 section of the 9x9 sudoku board/puzzle
    //that returns a new Array2 at each iteration
    pub fn get_subsquare(&self, start_row: u32, start_col: u32) -> Array2<T> {
        let mut sub_square_data = Vec::new();
        for row in start_row..start_row + 3 {
            for col in start_col..start_col + 3 {
                let index = (row * self.width + col) as usize;
                sub_square_data.push(self.data[index].clone());
            }
        }
        Array2::new(3, 3, sub_square_data)
    }

    //unique helper function that checks all elements in a given 1d vector
    pub fn is_unique(&self) -> bool {
        let mut unique_set = HashSet::new();

        for element in &self.data {
            if !unique_set.insert(element) {
                return false;
            }
        }

        true
    }

    /*
        Function: Solving Sudoku

        The paramter self takes in the Array2 object hat at each iteration of height uses 2 seperate hashmaps that
        are recycled at every iteration. Within the function iter_row_major is used to skip around the 1d vector
        to locate all elements based on the current coordinates of the short math equation. Which then takes the
        the amount of elements at the specifc row and enumerates returns the index of the elements of the row's 
        position in the board. Throgh the same process of iter_col_major, the if condition verifies the value within
        the tuple's for loop iteration if the element is within the range of 1..9 and is not a duplicate based on the 
        hashmaps at every iteration.

    */
    pub fn is_valid_sudoku(&self) -> bool {
        for i in 0..self.height {
            let mut row_numbers = HashSet::new();
            let mut col_numbers = HashSet::new();
    
            for (_col, row_element) in self.iter_row_major().skip((i * self.width) as usize).take(self.width as usize).enumerate() {
                if let Some(element) = row_element {
                    if element < 1 || element > 9 || !row_numbers.insert(element.clone()) {
                        //println!("!!Error!! => Invalid element in row {} and column {}: {}", i, _col, element);
                        return false;
                    }
                }
            }
    
            for (_row, col_element) in self.iter_col_major().skip(i as usize).step_by(self.height as usize).enumerate() {
                if let Some(element) = col_element {
                    if element < 1 || element > 9 || !col_numbers.insert(element.clone()) {
                        //println!("!!Error!! => Invalid element in row {} and column {}: {}", _row, i, element);
                        return false;
                    }
                }
            }
        }
        true
    }
}