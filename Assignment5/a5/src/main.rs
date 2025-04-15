use std::collections::BTreeMap;

fn add_polynomials(equation: &str) -> String {
    let mut equation: String = equation.replace(&['(', ')'][..], "");
    equation = equation.replace("+ ", "+").replace("- ", "-");
    let terms: Vec<&str> = equation.split_whitespace().collect();
    let mut values: BTreeMap<String, i32> = BTreeMap::new();

    for i in 0..terms.len() {
        let term: Vec<&str> = terms[i].split("**").collect();
        let mut negative: bool = false;
        let mut coefficient: String = term[0].to_string();
        let mut exponent: &str = "0";
        if term.len() == 2 {
            exponent = term[1];
        }
        else if coefficient.contains("x"){
            exponent = "1";
        }
        if coefficient.contains("-"){
            negative = true;
        }
        coefficient = coefficient.replace(&['+', '-'][..], "");
        if coefficient == "x" {
            coefficient = "1".to_string();
        }
        else{
            coefficient = coefficient.replace("x", "");
        }
        let mut val: i32 = coefficient.parse::<i32>().unwrap();
        if negative{
            val = val * -1;
        }
        let curr_val: i32 = values.remove(exponent).unwrap_or(0);
        values.insert(exponent.to_string(), val + curr_val);
    }

    let mut result: String = "".to_owned();

    for (exponent, coefficient) in &values {
        if exponent == "0" {
            result = coefficient.to_string();
        }
        else if exponent == "1" {
            if result.len() > 0 {
                result = coefficient.to_string() + "x  " + &result;
            }
            else {
                result = coefficient.to_string() + "x";
            }
        }
        else {
            if result.len() > 0 {
                result = coefficient.to_string() + "x**" + exponent + "  " + &result;
            }
            else {
                result = coefficient.to_string() + "x**" + exponent;
            }
        }
    }

    result = result.replace("  -", " - ").replace("  ", " + "). replace("1x", "x");
    result
}


// add two polynomials
// (x**3 + 5x**2  - 3x + 3) + (4x**5 – 2x**2 + 1)
// = 4x**5 + x**3 + 3x**2 - 3x + 4
fn main() {
    println!("{}", add_polynomials("(x**3 + 5x**2  - 3x + 3) + (4x**5 - 2x**2 + 1)"));
    println!("{}", add_polynomials("(2) + (2x**8)"));
}
