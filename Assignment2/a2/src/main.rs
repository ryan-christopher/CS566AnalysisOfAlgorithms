// max_sort takes as input an array of integers and returns the array sorted from smallest to largest
fn max_sort(arr: &mut [i32]) {
    println!("{:?}", arr);
    // set the end value to the last entry in the array
    let mut end_val = arr.len() - 1;
    // while the last value is not the beginning, repeat
    while end_val >= 1 { 
        // set curent max and index to first value of array
        let mut max = arr[0];
        let mut index = 0;
        // iterate until the end value of the unsorted vals
        for num in 0..=end_val {
            // find largest value and store corresponding index
            if arr[num] > max {
                max = arr[num];
                index = num;
            }
        }
        // create temp variable to store the last unsorted val, then replace with
        // then swap the last val with the max found
        let temp = arr[end_val];
        arr[end_val] = max;
        arr[index] = temp;
        println!("{:?}", arr);
        // decrement end_val by one as there is one more value sorted, continue until beginning of array
        end_val -= 1;
    }
}

fn main() {
    let mut arr: [i32; 5] = [5, 4, 3, 1, 2];
    max_sort(&mut arr);
}
 