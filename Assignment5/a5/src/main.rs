use std::collections::BTreeMap;

// add_polynomials operates in theta n, where n is the
// number of terms in the equation given as input

fn add_polynomials(equation: &str) -> String {
    // format input string, remove parentheses and whitespace after signs
    let mut equation: String = equation.replace(&['(', ')'][..], "");
    equation = equation.replace("+ ", "+").replace("- ", "-");
    // convert string to list of temrs
    let terms: Vec<&str> = equation.split_whitespace().collect();
    // create btree to store terms by exponent value
    let mut values: BTreeMap<i32, i32> = BTreeMap::new();

    // iterate through individual terms
    for i in 0..terms.len() {
        // split at ** to get exponent value
        let term: Vec<&str> = terms[i].split("**").collect();
        // set boolean variable negative to false by default
        let mut negative: bool = false;
        let mut coefficient: String = term[0].to_string();
        let mut exponent: i32 = 0;
        // if there was a value after **, set it to exponent
        if term.len() == 2 {
            exponent = term[1].parse::<i32>().unwrap();
        }
        // remove extra x if no leading coefficient num
        else if coefficient.contains("x"){
            exponent = 1;
        }
        // set negative to true to multiply by -1 later on
        if coefficient.contains("-"){
            negative = true;
        }
        // remove + and - signs
        coefficient = coefficient.replace(&['+', '-'][..], "");
        if coefficient == "x" {
            coefficient = "1".to_string();
        }
        else{
            coefficient = coefficient.replace("x", "");
        }
        // store coefficient as integer, multiply by -1 if original value
        // was negative
        let mut val: i32 = coefficient.parse::<i32>().unwrap();
        if negative{
            val = val * -1;
        }
        // if value exists already in btree, get it
        let curr_val: i32 = values.remove(&exponent).unwrap_or(0);
        // then add value and optional existing value to exponent's 
        // key in tree
        values.insert(exponent, val + curr_val);
    }

    // create empty output string to append to
    let mut result: String = "".to_owned();

    // iterate through btree, joining terms back together
    for (exponent, coefficient) in values.iter() {
        if *coefficient != 0 {
            // check for integers, then work way up through exponent values
            if *exponent == 0 {
                result = coefficient.to_string();
            }
            // handle x**1 values
            else if *exponent == 1 {
                if result.len() > 0 {
                    result = coefficient.to_string() + "x  " + &result;
                }
                else {
                    result = coefficient.to_string() + "x";
                }
            }
            // handle all other exponent values
            else {
                if result.len() > 0 {
                    result = coefficient.to_string() + "x**" + &exponent.to_string() + "  " + &result;
                }
                else {
                    result = coefficient.to_string() + "x**" + &exponent.to_string();
                }
            }
        }
    }
    // check if result is 0
    if result == "" {
        result = "0".to_string();
    }
    // adjust spacing to be uniform, account for 1x
    result = result.replace("  -", " - ").replace("  ", " + "). replace("1x", "x");
    // return result
    result
}



fn main() {
    println!("========== Test Case 1 ==========");
    println!("(x**3 + 5x**2  - 3x + 3) + (4x**5 - 2x**2 + 1)");
    println!("Expected: 4x**5 + x**3 + 3x**2 - 3x + 4");
    let test_case_1 = add_polynomials("(x**3 + 5x**2  - 3x + 3) + (4x**5 - 2x**2 + 1)");
    println!("Result:   {}\n", test_case_1);

    println!("========== Test Case 2 ==========");
    println!("(-14x**2 - 8x + 1) + (15x**2 - 8x -1)");
    println!("Expected: x**2 - 16x");
    let test_case_2 = add_polynomials("(-14x**2 - 8x + 1) + (15x**2 - 8x -1)");
    println!("Result:   {}\n", test_case_2);

    println!("========== Test Case 3 ==========");
    println!("(2 + 50x**9 - 13x**3 - 13x**4) + (37x**5 + 5 - 2x**4)");
    println!("Expected: 50x**9 + 37x**5 - 15x**4 - 13x**3 + 7");
    let test_case_3 = add_polynomials("(2 + 50x**9 - 13x**3 - 13x**4) + (37x**5 + 5 - 2x**4)");
    println!("Result:   {}\n", test_case_3);

    println!("========== Test Case 4 ==========");
    println!("(x + 2x**2 + 3x**3 + 4x**4 + 5x**5) + (-x - 2x**2 - 3x**3 - 4x**4 - 5x**5)");
    println!("Expected: 0");
    let test_case_4 = add_polynomials("(x + 2x**2 + 3x**3 + 4x**4 + 5x**5) + (-x - 2x**2 - 3x**3 - 4x**4 - 5x**5)");
    println!("Result:   {}\n", test_case_4);

    println!("========== Test Case 5 ==========");
    println!("(82x**20 + 523x**17 - 39x**5) + (195x**10 - 2x**8 - 620)");
    println!("Expected: 82x**20 + 523x**17 + 195x**10 - 2x**8 - 39x**5 - 620");
    let test_case_5 = add_polynomials("(82x**20 + 523x**17 - 39x**5) + (195x**10 - 2x**8 - 620)");
    println!("Result:   {}\n", test_case_5);
    
    println!("========== Test Case 6 ==========");
    println!("(-10x**200 - 125x**189 - x**99 - 55x - 20) + (-x**290 - 62x**53 - x**20 - x - 100)");
    println!("Expected: -x**290 - 10x**200 - 125x**189 - x**99 - 62x**53 - x**20 - 56x - 120");
    let test_case_6 = add_polynomials("(-10x**200 - 125x**189 - x**99 - 55x - 20) + (-x**290 - 62x**53 - x**20 - x - 100)");
    println!("Result:   {}\n", test_case_6);
}
