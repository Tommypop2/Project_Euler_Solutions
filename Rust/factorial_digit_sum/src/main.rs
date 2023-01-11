extern crate num;
use num::{BigInt, bigint::ToBigInt};
fn compute_sum(n: u32) -> u32 {
    let mut total: BigInt = 1.to_bigint().unwrap();
    
    for i in 1..n + 1 {
        // loop {
        //     if total % 10 == 0 {
        //         total /= 10;
        //         continue;
        //     }
        //     break
        // }
        total *= i;
    }
    let mut res = 0;
    for i in total.to_string().chars() {
        res += i.to_string().parse::<u32>().unwrap();
    }
    res
}

fn main() {
    println!("{}", compute_sum(100));
}
