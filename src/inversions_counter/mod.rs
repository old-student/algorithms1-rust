use std::fs::File;
use std::io::{self,BufRead};
use std::cmp::Ordering;

pub struct InversionsCounter {
    data: Vec<i32>,
}
    
impl InversionsCounter {
    
    pub fn new() -> InversionsCounter {
        InversionsCounter {
            data: Vec::new(),
        }
    }
        
    fn get_element(&self, index: usize) -> i32 {
        match self.data.get(index) {
            Some(element_ref) => *element_ref,
            None => panic!("Invalid index by using vector"),
        }
    }
    
    fn set_element(&mut self, index: usize, value: i32) {
        match self.data.get_mut(index) {
            Some(element_ref) =>  *element_ref = value,
            None => panic!("Invalid index by using vector"),
        };
    }
   
    pub fn set_data(&mut self, file_name: &str) {
        let file = File::open(file_name).ok().expect("Error by opening input file");
        let reader = io::BufReader::new(file);
        self.data = reader.lines().map(|line| line.unwrap().trim().parse::<i32>()
                                       .ok().expect("Couldn't convert string into an integer {}")
                                       ).collect();
    }
    
    pub fn get_inversions_number(&mut self) -> u64 {
        let size = self.data.len();
        self.count_inversions(0, size - 1)
    }
    
    fn count_inversions(&mut self, left: usize, right: usize) -> u64 {
        match left.cmp(&right) {
            Ordering::Greater | Ordering::Equal => 0,
            Ordering::Less => {
                let middle = (left + right) / 2;
                self.count_inversions(left, middle)
                    + self.count_inversions(middle + 1, right)
                    + self.count_split_inversions(left, middle, middle + 1, right)
            }
        }
    }

    fn count_split_inversions(&mut self, 
                              begin_left: usize, end_left: usize, 
                              begin_right: usize, end_right: usize) -> u64 {
        //prepare left vector
        let mut left_vec: Vec<_> = (begin_left..end_left+1).rev().map(|i| self.get_element(i)).collect();
        //prepare right vector
        let mut right_vec: Vec<_> = (begin_right..end_right+1).rev().map(|i| self.get_element(i)).collect();
        //init inversion counter by zero
        let mut split_inv = 0;
        for i in (begin_left..end_right+1) {
            if left_vec.is_empty() {
                self.set_element(i, right_vec.pop().unwrap());
            }
            else if right_vec.is_empty() || (left_vec.last() <= right_vec.last()) {
                self.set_element(i, left_vec.pop().unwrap());
            }
            else {
                self.set_element(i, right_vec.pop().unwrap());
                split_inv += left_vec.len() as u64;
            }
        }            
        split_inv
    }
}
