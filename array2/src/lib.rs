use std::collections::HashSet;

pub struct Array2<T: Clone> {
    #[allow(dead_code)]
    width: u32,
    #[allow(dead_code)]
    height: u32,
    data: Vec<T>,
}

impl<T: Clone + std::hash::Hash + std::cmp::Eq> Array2<T> {
    /// Creates a new `Array2`.
    ///
    /// # Arguments
    ///
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`.
    pub fn new(row: u32, column: u32, data: Vec<T>) -> Self {
        Array2 {
            width: column,
            height: row,
            data,
        }
    }

    /*------------------------\
    |    Public Functions     |
    \------------------------*/

    pub fn iter_row_major(&self) -> bool {
        let mut unique_set = HashSet::new(); // HashSet to track unique elements

        for element in &self.data {
            if unique_set.contains(element) {
                // If the element is already in the set, it's not unique
                return false;
            } else {
                unique_set.insert(element.clone()); // Insert the element into the set
            }
        }

        true // All elements are unique
    }

    pub fn iter_col_major(&self) -> bool {
        let mut unique_set = HashSet::new(); // HashSet to track unique elements

        for element in &self.data {
            if unique_set.contains(element) {
                // If the element is already in the set, it's not unique
                return false;
            } else {
                unique_set.insert(element.clone()); // Insert the element into the set
            }
        }

        true // All elements are unique
    }

    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn iter_subsquare_major(&self) -> bool {
        let mut unique_set = HashSet::new(); // HashSet to track unique elements

        for element in &self.data {
            if unique_set.contains(element) {
                // If the element is already in the set, it's not unique
                return false;
            } else {
                unique_set.insert(element.clone()); // Insert the element into the set
            }
        }

        true // All elements are unique
    }
}