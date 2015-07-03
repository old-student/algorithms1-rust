extern crate time;

mod inversions_counter;

fn main() {
    let mut inv_counter = inversions_counter::InversionsCounter::new();
    inv_counter.set_data("test_data/inversions_counter/IntegerArray.txt");
    
    let start = time::now();
    let inv_number = inv_counter.get_inversions_number();
    let finish = time::now();
    
    println!("Number of inversions {}", inv_number);
    println!("Needed time {} nsec", finish.tm_nsec - start.tm_nsec);
}
