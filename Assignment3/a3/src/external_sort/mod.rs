use std::collections::VecDeque;
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
    let temp_unsorted_dir = fs::create_dir("./temp_unsorted_files");
    let mut counter = 0;
    let mut temp_file_number = 1;
    let mut temp_file_name: String = "./temp_unsorted_files/temp".to_owned();
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

    let temp_sorted_dir = fs::create_dir("./temp_sorted_files");
    // can delete this later, temp file number will still be at 100
    let temp_file_number = 100;
    /*
    let mut temp_sorted_file_number = 1;
    

    while temp_sorted_file_number <= temp_file_number {
        // can remove let mut once done, will still be created
        let mut temp_sorted_file_name: String = "./temp_sorted_files/temp".to_owned();
        temp_sorted_file_name.push_str(&temp_sorted_file_number.to_string());
        temp_sorted_file_name.push_str(".txt");
        let mut temp_unsorted_file_name: String = "./temp_unsorted_files/temp".to_owned();
        temp_unsorted_file_name.push_str(&temp_sorted_file_number.to_string());
        temp_unsorted_file_name.push_str(".txt");

        // can remove let mut later
        let mut temp_sorted_file = fs::OpenOptions::new().append(true).create(true).open(&temp_sorted_file_name).expect("Can not open file.");
        let contents = fs::read_to_string(&temp_unsorted_file_name).expect("txt file");
        // mergesort 
        if method == "mergesort" {
            let values: Vec<i32> = contents.trim().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
            //let values = merge_sort(&values).into_iter().map(|i| i.to_string()).collect::<String>();
            let sort_values: String = merge_sort(&values).into_iter().map(|i| i.to_string() + " ").collect();
            temp_sorted_file.write_all(sort_values.as_bytes()).expect("Can not write to file.");
            //println!("{:?}", sorted_vals);
        }
        // heapsort
        else if method == "heapsort" {
            let mut values: Vec<i32> = contents.trim().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
            heapsort(&mut values);
            //println!("{:?}", values);
        }


        // quicksort
        else if method == "quicksort" {
            let mut values: Vec<i32> = contents.trim().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
            let mut vals_len = values.len();
            quick_sort(&mut values, 0, vals_len - 1);
            //println!("{:?}", values);
        }

        // binary tree sort
        else if method == "btree" {
            
        }
        temp_sorted_file_number += 1;
}
        */

        
    let mut sorted_file_list: Vec<VecDeque<i32>> = Vec::new();
    for i in 1..=temp_file_number{
        let mut temp_sorted_file_name: String = "./temp_sorted_files/temp".to_owned();
        temp_sorted_file_name.push_str(&i.to_string());
        temp_sorted_file_name.push_str(".txt");
        let contents = fs::read_to_string(&temp_sorted_file_name).expect("txt file");
        let values: VecDeque<i32> = contents.trim().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
        sorted_file_list.push(values);
    }

    let mut sorted_vals = fs::OpenOptions::new().append(true).create(true).open(&"sorted_vals").expect("Can not open file.");
    let mut merging_array: [Option<i32>; 100] = [Some(0);100];
    let mut sorted = false;
    while sorted == false {
        let mut emptied_lists = 0;
    for i in 0..merging_array.len() {
        
        if merging_array[i] == Some(0) {
            merging_array[i] = sorted_file_list[i].pop_front();
        }
        else if merging_array[i] == None {
            emptied_lists += 1;
            sorted = true;
            println!("{:?}", merging_array);
            break;
        }
        if emptied_lists == merging_array.len(){
            sorted = true;
            break;
        }
    }

    //let min = merging_array.iter().min().unwrap().unwrap();
    let min_index = merging_array.iter().position(|x| x == merging_array.iter().min().unwrap()).unwrap();
    //println!("{:?}", merging_array.iter().min().unwrap().unwrap());
    sorted_vals.write_all((merging_array[min_index].unwrap().to_string() + ", ").as_bytes());
    merging_array[min_index] = Some(0);
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
