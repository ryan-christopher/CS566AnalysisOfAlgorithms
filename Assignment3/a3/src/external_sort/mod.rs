use std::fs;
use std::fs::read_to_string;
use std::io::Write;
use std::time::Instant;

// will need support for: mergesort, heapsort, quicksort, 
// and binary tree based algorithms

pub fn sort(method: &str, file: &str) {  
    let now = Instant::now();
    
    //if method == "mergesort" {
    //    println!("AA");
    //}
    // ===== Go through large file, create smaller ones =====
    /* 
    let temp_dir = fs::create_dir("./temp_sort_files");
    let mut counter = 0;
    let mut temp_file_number = 1;
    let mut temp_file_name: String = "./temp_sort_files/temp".to_owned();
    temp_file_name.push_str(&temp_file_number.to_string());
    let mut temp_file = fs::OpenOptions::new().append(true).create(true).open(&temp_file_name).expect("Can not open file.");
    for line in read_to_string(file).unwrap().lines() {
        temp_file.write_all(line.as_bytes()).expect("Can not write to file.");
        counter += 1;
        if counter >= 500 {
            counter = 0;
            temp_file_number += 1;
            temp_file_name = "./temp_sort_files/temp".to_owned();
            temp_file_name.push_str(&temp_file_number.to_string());
            temp_file = fs::OpenOptions::new().append(true).create(true).open(&temp_file_name).expect("Can not open file.");
        } 
    }
    */
    // ======================================================

    

    println!("This thing on?");
    let elapsed = now.elapsed();
    println!("{:?}", elapsed);
    
}
