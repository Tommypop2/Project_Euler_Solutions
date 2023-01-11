use std::fs;
fn prime_factor_decomposition(num: u64) -> Vec<u64> {
    // Sieve of Eratosthenes
    let mut primes: Vec<u64> = vec![1; (num + 1).try_into().unwrap()]; // Index denotes the number, and the value denotes whether or not it is prime
                                                                       // At first, assume that every number is prime
    let mut p = 2;
    while p * p <= num + 1 {
        for i in (p * p..num + 1).step_by(p.try_into().unwrap()) {
            primes[i as usize] = 0;
        }
        p += 1;
    }
    let mut current = num;
    let mut prime_factors: Vec<u64> = vec![];
    if primes[num as usize - 1] == 1 {
        return vec![num];
    }
    loop {
        for i in 2..current + 1 {
            if primes[i as usize] == 1 && current % i == 0 {
                current /= i;
                prime_factors.push(i);
                break;
            }
        }
        if current == 1 {
            break;
        }
    }
    return prime_factors;
}
fn get_greatest_possible_factor(prime_factors: &Vec<u64>, max: &u64) -> usize {
    let mut length = 1;
    for i in prime_factors {
        length *= i;
    }
    let max_size: u64 = max / length;
    let mut largest_possible: u64 = 1;
    for i in prime_factors {
        if *i < *&max_size.to_string().len() as u64 {
            largest_possible = *i;
        }
    }
    return largest_possible as usize;
}
fn main() {
    let file_contents = fs::read_to_string("numbers.txt").expect("Should be able to read file");
    let lines = file_contents.split("\n");
    let lines_vec = lines.collect::<Vec<_>>();
    let prime_factors = prime_factor_decomposition((lines_vec[0].len() - 1).try_into().unwrap());
    // let max_prime_factor = *prime_factors.iter().max().unwrap();

    let factor_to_split_into = get_greatest_possible_factor(&prime_factors, &u64::max_value());
    let num_of_splits = (lines_vec[0].len() - 1) / factor_to_split_into;
    // let mut numbers: [[u64; 5]; 100] = [[0; 5]; 100]; This solution is technically more performant if the file's number of lines and length per line are known at compile time
    let mut numbers = vec![vec![0; num_of_splits]; lines_vec.len()];
    for i in 0..lines_vec.len() {
        let mut start: usize = 0;
        let mut end: usize = factor_to_split_into;
        for n in 0..num_of_splits {
            numbers[i][n as usize] = lines_vec[i][start..end].parse::<u64>().unwrap();
            start += factor_to_split_into;
            end += factor_to_split_into;
        }
    }
    let mut carry = 0;
    let mut totals: Vec<String> = vec![];
    for n in (0..numbers[0].len()).rev() {
        let mut total = carry;
        for i in &numbers {
            total += i[n];
        }
        let total_str = total.to_string();
        let mut end = total_str.len();
        if total_str.len() > factor_to_split_into {
            end = total_str.len() - factor_to_split_into;
            carry = total_str[0..end].parse::<u64>().unwrap();
        }
        totals.push(total_str.replace(&total_str[0..end].to_string(), ""));
    }
    let totals_length = totals.len();
    totals[totals_length - 1] = carry.to_string() + &totals[totals_length - 1];
    totals.reverse();
    let mut totals_str: String = "".to_string();
    for i in totals {
        totals_str += &i;
    }
    println!("{}", &totals_str[0..10]);
}
