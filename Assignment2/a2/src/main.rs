// Problem 4:
// max_sort takes as input an array of integers and returns the array sorted from smallest to largest
fn max_sort(arr: &mut [i32]) {
    println!("Starting:\n{:?}", arr);
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
        // decrement end_val by one as there is one more value sorted, continue until beginning of array
        end_val -= 1;
        println!("{:?}", arr);
    }
    println!("Sorted:\n{:?}", arr);
}


// Problem 5:
// roman_numeral takes as input a decimal value and ouptuts the corresponding Roman numeral
// representation of the input
fn roman_numeral(mut value: i32) -> String{
    // set roman numeral to empty string
    let mut roman_num: String = "".to_owned();
    // create arrays for representation values and representations
    let rep_vals: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let rep: [&str; 13] = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    // until value is 0, find largest match in rep_vals and add the corresponding rep to 
    // return value
    while value > 0 {
        for num in 0..=rep_vals.len() - 1 {
            if value >= rep_vals[num] {
                roman_num.push_str(rep[num]);
                value -= rep_vals[num];
                break;
            }
        }
    }
    // return roman numeral representation
    roman_num.to_string()
}


fn main() {
    // test cases for problem 4
    // test case 1 - sorted list
    let mut arr1: [i32; 5] = [1, 2, 3, 4, 5];
    max_sort(&mut arr1);
    // test case 2 - reversed list
    let mut arr2: [i32; 10] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    max_sort(&mut arr2);
    // test case 3 - random values
    let mut arr3: [i32; 7] = [12, 2, 58, 9, 100, 57, 79];
    max_sort(&mut arr3);

    // test cases for problem 5
    // 2025 = MMXXV
    println!("Decimal: 2025\nRoman  : {}", roman_numeral(2025));
    // 3998 = MMMCMXCVIII
    println!("Decimal: 3998\nRoman  : {}", roman_numeral(3998));
    // 5999 = MMMMMCMXCIX
    println!("Decimal: 5999\nRoman  : {}", roman_numeral(5999));
}