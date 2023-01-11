fn count_factors(num: u32) -> u32 {
    if num == 1 {
        return 1;
    }
    let mut total: u32 = 2; //Assume that the number is divisible by both 1 and itself
    let max: u32 = ((num as f32).powf(1.0 / 2.0) + 1.0) as u32;
    for i in 2..max {
        if num % i == 0 {
            total += 2;
        }
    }
    total
}
fn main() {
    let mut n = 1;
    let mut num = 0;
    loop {
        num += n;
        if count_factors(num) > 500 {
            println!("{}", num);
            break;
        }
        n += 1;
    }
}
