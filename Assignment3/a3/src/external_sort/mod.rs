use std::fs;
use std::fs::read_to_string;
use std::io::Write;
use std::time::Instant;

// will need support for: mergesort, heapsort, quicksort, 
// and binary tree based algorithms

pub fn sort(method: &str, file: &str) {  
    let now = Instant::now();
    // ===== Go through large file, create smaller ones =====
    /*
    let temp_dir = fs::create_dir("./temp_sort_files");
    let mut counter = 0;
    let mut temp_file_number = 1;
    let mut temp_file_name: String = "./temp_sort_files/temp".to_owned();
    temp_file_name.push_str(&temp_file_number.to_string());
    temp_file_name.push_str(".txt");
    let mut temp_file = fs::OpenOptions::new().append(true).create(true).open(&temp_file_name).expect("Can not open file.");
    for line in read_to_string(file).unwrap().lines() {
        temp_file.write_all(line.as_bytes()).expect("Can not write to file.");
        counter += 1;
        if counter > 500 {
            counter = 0;
            temp_file_number += 1;
            temp_file_name = "./temp_sort_files/temp".to_owned();
            temp_file_name.push_str(&temp_file_number.to_string());
            temp_file_name.push_str(".txt");
            temp_file = fs::OpenOptions::new().append(true).create(true).open(&temp_file_name).expect("Can not open file.");
        } 
    }
    */
    // ======================================================

    // mergesort 
    if method == "mergesort" {
        let mut contents = fs::read_to_string("./temp_sort_files/temp0.txt").expect("txt file");
        println!("{}", contents);
        let values: Vec<i32> = contents.trim().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
        println!("{:?}", values);
        println!("Sorting via mergesort");
        let sorted_vals: Vec<i32> = merge_sort(&values);
        println!("{:?}", sorted_vals);
    }
    // heapsort
    if method == "heapsort" {
        let mut contents = fs::read_to_string("./temp_sort_files/temp0.txt").expect("txt file");
        println!("{}", contents);
        let mut values: Vec<i32> = contents.trim().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
        println!("{:?}", values);
        println!("Sorting via heapsort");
        heapsort(&mut values);
        println!("{:?}", values);
    }


    // quicksort
    if method == "quicksort" {
        let mut contents = fs::read_to_string("./temp_sort_files/temp0.txt").expect("txt file");
        println!("{}", contents);
        let mut values: Vec<i32> = contents.trim().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
        println!("{:?}", values);
        println!("Sorting via quicksort");
        let mut vals_len = values.len();
        quick_sort(&mut values, 0, vals_len - 1);
        println!("{:?}", values);
    }

    // binary tree sort
    if method == "btree" {
        
    }

    let elapsed = now.elapsed();
    println!("{:?}", elapsed);
    
}

// ===== Merge Sort =====
fn merge_sort(vec: &Vec<i32>) -> Vec<i32> {
    if vec.len() < 2 {
        vec.to_vec()
    } else {
        let size = vec.len() / 2;
        let left = merge_sort(&vec[0..size].to_vec());
        let right = merge_sort(&vec[size..].to_vec());
        let merged = merge(&left, &right);

        merged
    }
}

fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<i32> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i = i + 1;
        } else {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i = i + 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    merged
}

// ===== Heap Sort =====
fn heapsort<T>(arr: &mut [T])
where
    T: Copy + Clone + PartialEq + PartialOrd,
{
    let end = arr.len();
    for start in (0..end / 2).rev() {
        reheap(arr, start, end - 1);
    }

    for end in (1..arr.len()).rev() {
        arr.swap(end, 0);
        reheap(arr, 0, end - 1);
    }
}

fn reheap<T>(arr: &mut [T], start: usize, end: usize)
where
    T: Copy + Clone + PartialEq + PartialOrd,
{
    let mut root = start;
    loop {
        let mut child = root * 2 + 1;
        if child > end {
            break;
        }

        if child < end && arr[child] < arr[child + 1] {
            child += 1;
        }
        if arr[root] < arr[child] {
            arr.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}

// ===== Quick Sort =====
fn quick_sort(slice: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivot = sub_sort(slice, low, high);
        if pivot > 0 {
            quick_sort(slice, low, pivot - 1)
        }
        quick_sort(slice, pivot + 1, high);
    }
}

pub fn sub_sort(slice: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = high;
    let mut i = low as isize - 1;

    for j in low..high {
        // println!("J now is: {}", j);
        if slice[j] < slice[pivot] {
            i += 1;
            slice.swap(i as usize, j);
        }
    }
    slice.swap((i + 1) as usize, pivot);
    (i + 1) as usize
}

// ===== Binary Tree Sort =====
