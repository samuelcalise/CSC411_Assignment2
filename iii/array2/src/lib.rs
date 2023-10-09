use std::collections::HashSet;

pub struct Array2<T: Clone> {
    width: u32,
    height: u32,
    data: Vec<T>,
}

pub struct RowMajorIterator<'a, T: Clone> {
    array: &'a Array2<T>,
    row: u32,
    col: u32,
}

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

pub struct ColMajorIterator<'a, T: Clone> {
    array: &'a Array2<T>,
    row: u32,
    col: u32,
}

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

impl<T: Clone + std::hash::Hash + std::cmp::Eq + std::cmp::PartialOrd<i32> + std::fmt::Display> Array2<T> {
    pub fn new(row: u32, column: u32, data: Vec<T>) -> Self {
        Array2 {
            width: column,
            height: row,
            data,
        }
    }

    pub fn iter_row_major(&self) -> RowMajorIterator<T> {
        RowMajorIterator {
            array: self,
            row: 0,
            col: 0,
        }
    }

    pub fn iter_col_major(&self) -> ColMajorIterator<T> {
        ColMajorIterator {
            array: self,
            row: 0,
            col: 0,
        }
    }

    pub fn iter_subsquare_major(&self) -> bool {
        let mut unique_set = HashSet::new();

        for element in &self.data {
            if !unique_set.insert(element) {
                return false;
            }
        }

        true
    }

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

    pub fn is_unique(&self) -> bool {
        let mut unique_set = HashSet::new();

        for element in &self.data {
            if !unique_set.insert(element) {
                return false;
            }
        }

        true
    }

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