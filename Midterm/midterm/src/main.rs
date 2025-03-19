use std::time::Instant;

// take as input n and k, return an integer of the 
// possible combinations of n items taken k at a time
fn choose_recursive(n: i32, k: i32) -> i32 {
    // base case, return 1
    if n == k || k == 0 {
        1
    }
    // recursive call
    else {
        choose_recursive(n - 1, k - 1) + choose_recursive(n - 1, k)
    }
}

fn choose_iterative(n: i64, k: i64) -> i64 {
    let mut n_fac: i64 = n;
    // iteratively calculate factorial value for n, k, and n - k
    for x in 1..n {
        n_fac *= x;
    }
    let mut k_fac: i64 = k;
    for y in 1..k {
        k_fac *= y;
    }
    let mut n_sub_k_fac: i64 = n - k;
    for z in 1..(n - k) {
        n_sub_k_fac *= z;
    }
    // return n! / (k! * (n - k)!)
    n_fac / (k_fac * n_sub_k_fac)
}

fn main() {
    println!("=== n = 6 | k = 3 ===");
    println!("Recursive function:");
    let mut now = Instant::now();
    println!("{} combinations", choose_recursive(6, 3));
    println!("Running time - {:?}", now.elapsed());
    println!("---------------------");
    println!("Iterative function:");
    now = Instant::now();
    println!("{} combinations", choose_iterative(6, 3));
    println!("Running time - {:?}", now.elapsed());
    
    println!("\n=== n = 10 | k = 5 ===");
    println!("Recursive function:");
    now = Instant::now();
    println!("{} combinations", choose_recursive(10, 5));
    println!("Running time - {:?}", now.elapsed());
    println!("---------------------");
    println!("Iterative function:");
    now = Instant::now();
    println!("{} combinations", choose_iterative(10, 5));
    println!("Running time - {:?}", now.elapsed());

    println!("\n=== n = 16 | k = 8 ===");
    println!("Recursive function:");
    now = Instant::now();
    println!("{} combinations", choose_recursive(16, 8));
    println!("Running time - {:?}", now.elapsed());
    println!("---------------------");
    println!("Iterative function:");
    now = Instant::now();
    println!("{} combinations", choose_iterative(16, 8));
    println!("Running time - {:?}", now.elapsed());
}
