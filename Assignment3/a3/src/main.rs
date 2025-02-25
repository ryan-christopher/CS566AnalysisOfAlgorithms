mod stack;
mod external_sort;
use std::{fs, io::Write};
use rand::Rng;

//use rand::Rng;


fn main() {
    //stack::stack_tests();
    external_sort::sort("quicksort", "./rand_nums.txt");
    //let mut file = File::open("./rand_nums.txt");
    // ===== code used to generate large unsorted file =====
    /*
    let mut file = fs::OpenOptions::new().append(true).create(true).open("./rand_nums.txt").expect("Can not open file.");
    for x in 1..=50000 {
        let mut curr_line: String = "".to_string();
        for y in 1..=500 {
            curr_line += &rand::rng().random_range(1..100000000).to_string();
            curr_line += " ";
        }
        curr_line += "\n";
        file.write_all(curr_line.as_bytes()).expect("Can not write to file.");
    }
    */
    // =====================================================
    //let contents = fs::read_to_string("./rand_nums.txt").expect("txt file");
    //println!("{}", contents);
}