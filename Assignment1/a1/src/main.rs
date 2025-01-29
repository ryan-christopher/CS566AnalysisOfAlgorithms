// function to help calculate the answer to #1
fn main() {
    let mut n: f32 = 0.0;
    let mut alg1: f32;
    let mut alg2: f32;
    println!("  n | faster | a1val | a2val");
    loop{
        n += 1.0;
        alg1 = n.powf(2.0) + n;
        alg2 = 14.0 * n.powf(1.25) + 10.0;

        if alg1 < alg2 {
            println!("{:3} | alg1   | {:5} | {alg2}", n, alg1)
        }
        else {
            println!("{:3} | alg2   | {:5} | {alg2}", n, alg1)
        }
        if n >= 100.0 {
            break
        }
    }
}
