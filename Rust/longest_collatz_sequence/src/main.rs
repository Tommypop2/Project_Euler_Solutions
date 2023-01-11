fn gen_sequence(mut n: u64) -> u64 {
    let mut length = 1;
    loop {
        if n == 1 {
            return length;
        }
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = (3 * n) + 1
        }
        length += 1;
    }
}

fn main() {
    let mut longest_chain = 0;
    let mut longest_chain_start = 1;
    for i in 1..1000001{
        let sequence_len = gen_sequence(i);
        if sequence_len > longest_chain {
            longest_chain = sequence_len;
            longest_chain_start = i;
        }
    }
    println!("{}", longest_chain_start)
}
