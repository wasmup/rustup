fn main() {
    // cargo run 5
    match std::env::args().skip(1).next() {
        Some(s) => {
            let n = s.parse(); //.expect("number expected");
            if n.is_err() {
                return;
            }
            println!("{}", factorial(n.unwrap())); // 120
        }
        None => return,
    }
}

fn factorial(mut n: u64) -> u64 {
    let mut f: u64 = 1;
    while n > 1 {
        f *= n;
        n -= 1;
    }
    f
}
