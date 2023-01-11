fn get_factors(n: i32) -> Vec<i32> {
    let mut factors: Vec<i32> = Vec::new();
    factors.push(1);
    for i in 2..(n as f64).powf(1.0 / 2.0) as i32 + 1 {
        if n % i == 0 {
            factors.push(i);
            factors.push(n / i);
        }
    }
    return factors;
}

fn main() {
    let mut total: i32 = 0;
    for i in 1..10000 {
        let factor_sum: i32 = get_factors(i).iter().sum();
        if get_factors(factor_sum).iter().sum::<i32>() == i {
            if factor_sum == i {
                continue;
            }
            total += i;
            println!("{}, {}", i, factor_sum);
        }
    }
    println!("{}", total);
}
