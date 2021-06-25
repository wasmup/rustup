fn main() {
    println!("Hello, world!");

    let c = gcd(3 * 7, 2 * 3 * 7);
    println!("gcd {}", c); // 21

    let c = gcd(2 * 3 * 7, 3 * 7);
    println!("gcd {}", c); // 21

    let c = gcd(0, 1);
    println!("gcd {}", c); // 0

    let c = gcd(0, 0);
    println!("gcd {}", c); // 0
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
