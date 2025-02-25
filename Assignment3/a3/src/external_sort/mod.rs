use std::collections::VecDeque;
use std::fmt::write;
use std::fs;
use std::fs::read_to_string;
use std::io::Write;
use std::time::Instant;

pub fn sort(method: &str, file: &str) {  
    let now = Instant::now();
    // ===== Go through large file, create smaller ones =====
    fs::create_dir("./temp_unsorted_files").expect("Error creating unsorted folder.");
    let mut counter = 0;
    let mut temp_file_number = 1;
    let mut temp_file_name: String = "./temp_unsorted_files/temp".to_owned();
    temp_file_name.push_str(&temp_file_number.to_string());
    temp_file_name.push_str(".txt");
    let mut temp_file = fs::OpenOptions::new().append(true).create(true).open(&temp_file_name).expect("Can not open file.");
    for line in read_to_string(file).unwrap().lines() {
        temp_file.write_all(line.as_bytes()).expect("Can not write to file.");
        counter += 1;
        if counter > 5000 {
            counter = 0;
            temp_file_number += 1;
            temp_file_name = "./temp_unsorted_files/temp".to_owned();
            temp_file_name.push_str(&temp_file_number.to_string());
            temp_file_name.push_str(".txt");
            temp_file = fs::OpenOptions::new().append(true).create(true).open(&temp_file_name).expect("Can not open file.");
        } 
    }
    // ======================================================


    fs::create_dir("./temp_sorted_files").expect("Error creating sorted folder.");
    
    let mut temp_sorted_file_number = 1;
    while temp_sorted_file_number <= temp_file_number {
        // can remove let mut once done, will still be created
        let mut temp_sorted_file_name: String = "./temp_sorted_files/temp".to_owned();
        temp_sorted_file_name.push_str(&temp_sorted_file_number.to_string());
        temp_sorted_file_name.push_str(".txt");
        let mut temp_unsorted_file_name: String = "./temp_unsorted_files/temp".to_owned();
        temp_unsorted_file_name.push_str(&temp_sorted_file_number.to_string());
        temp_unsorted_file_name.push_str(".txt");


        let mut temp_sorted_file = fs::OpenOptions::new().append(true).create(true).open(&temp_sorted_file_name).expect("Can not open file.");
        let contents = fs::read_to_string(&temp_unsorted_file_name).expect("txt file");
        // mergesort 
        if method == "mergesort" {
            let values: Vec<i32> = contents.trim().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
            let sort_values: String = merge_sort(&values).into_iter().map(|i| i.to_string() + " ").collect();
            
            temp_sorted_file.write_all(sort_values.as_bytes()).expect("Mergesort error.");
        }
        // heapsort
        else if method == "heapsort" {
            let mut values: Vec<i32> = contents.trim().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
            heapsort(&mut values);
            
        }


        // quicksort
        else if method == "quicksort" {
            let mut values: Vec<i32> = contents.trim().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
            let vals_len = values.len() - 1;
            quick_sort(&mut values, 0, vals_len);
            let sort_values: String = values.into_iter().map(|i| i.to_string() + " ").collect();
            temp_sorted_file.write_all(sort_values.as_bytes()).expect("Quicksort error.");
        }

        // binary tree sort
        else if method == "btree" {
            
        }
        temp_sorted_file_number += 1;
}
        

        
    let mut sorted_file_list: Vec<VecDeque<i32>> = Vec::new();
    for i in 1..=temp_file_number{
        let mut temp_sorted_file_name: String = "./temp_sorted_files/temp".to_owned();
        temp_sorted_file_name.push_str(&i.to_string());
        temp_sorted_file_name.push_str(".txt");
        let contents = fs::read_to_string(&temp_sorted_file_name).expect("txt file");
        let values: VecDeque<i32> = contents.trim().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
        sorted_file_list.push(values);
    }

    let mut sorted_vals = fs::OpenOptions::new().append(true).create(true).open(&"sorted_vals.txt").expect("Can not open file.");
    let mut merging_array: [Option<i32>; 100] = [Some(0);100];
    let sorted = false;

    for i in 0..merging_array.len() {
        if merging_array[i] == Some(0) && i < sorted_file_list.len() {
            merging_array[i] = sorted_file_list[i].pop_front();
        }
        else if merging_array[i] == Some(0) && i >= sorted_file_list.len() {
            merging_array[i] = None;
        }
    }
    let mut write_count = 0;
    let mut values_line = "".to_owned();
    while sorted == false {
        let mut min_index: isize = -1;
        for element in 0..merging_array.len() {
            let curr_element = merging_array[element];
            if curr_element != None && min_index == -1 {
                min_index = element as isize;
            }
            else if curr_element != None && curr_element < merging_array[min_index as usize] {
                min_index = element as isize;
            }
        }
        if min_index == -1 {
            sorted_vals.write_all(values_line.as_bytes()).expect("Unable to write to sorted file.");
            break;
        }
        if write_count < 1000 {
            write_count += 1;
            values_line += &merging_array[min_index as usize].unwrap().to_string();
            values_line += "\n";
        }
        else{
            write_count = 0;
            values_line += &merging_array[min_index as usize].unwrap().to_string();
            sorted_vals.write_all(values_line.as_bytes()).expect("Unable to write to sorted file.");
            values_line = "\n".to_owned();
        }
        merging_array[min_index as usize] = sorted_file_list[min_index as usize].pop_front();
    }



    let elapsed = now.elapsed();
    println!("{:?}", elapsed);
    
}

// ===== Merge Sort =====
fn merge_sort(nums: &Vec<i32>) -> Vec<i32> {
    if nums.len() < 2 {
        nums.to_vec()
    } else {
        let size = nums.len() / 2;
        let left_half = merge_sort(&nums[0..size].to_vec());
        let right_half = merge_sort(&nums[size..].to_vec());
        let merged = merge(&left_half, &right_half);
        merged
    }
}

fn merge(left_half: &Vec<i32>, right_half: &Vec<i32>) -> Vec<i32> {
    let mut merged_vals: Vec<i32> = Vec::new();
    let mut x = 0;
    let mut y = 0;
    
    while x < left_half.len() && y < right_half.len() {
        if left_half[x] < right_half[y] {
            merged_vals.push(left_half[x]);
            x = x + 1;
        } else {
            merged_vals.push(right_half[y]);
            y = y + 1;
        }
    }

    if x < left_half.len() {
        while x < left_half.len() {
            merged_vals.push(left_half[x]);
            x = x + 1;
        }
    }

    if y < right_half.len() {
        while y < right_half.len() {
            merged_vals.push(right_half[y]);
            y = y + 1;
        }
    }

    merged_vals
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
fn quick_sort(nums: &mut [i32], start: usize, end: usize) {
    if start < end {
        let pivot = sort_pivot(nums, start, end);
        if pivot > 0 {
            quick_sort(nums, start, pivot - 1)
        }
        quick_sort(nums, pivot + 1, end);
    }
}

fn sort_pivot(nums: &mut [i32], start: usize, end: usize) -> usize {
    let pivot = end;
    let mut index = start as isize - 1;

    for curr_element in start..end {
        // println!("J now is: {}", j);
        if nums[curr_element] < nums[pivot] {
            index += 1;
            nums.swap(index as usize, curr_element);
        }
    }
    nums.swap((index + 1) as usize, pivot);
    (index + 1) as usize
}

// ===== Binary Tree Sort =====
