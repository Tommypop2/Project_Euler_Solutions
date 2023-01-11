extern crate num;
use num::{BigInt};
use num::bigint::{ToBigInt};
fn gen_sum(base: u32, exponent: usize) -> u32 {
    let mut total: u32 = 0;
    let number: BigInt = base.to_bigint().unwrap();
    let result = num::pow(number, exponent);
    for i in result.to_string().chars() {
        total += i
            .to_string()
            .parse::<u32>()
            .expect("Char couldn't be parsed to int");
    }
    return total;
}

fn main() {
    
    println!("{}", gen_sum(2, 1000))
    
}
