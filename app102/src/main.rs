fn main() {
    let s = "102";
    let i: i32 = s.parse().unwrap();

    println!("i = {}", i);

    use std::env;

    // Prints each argument on a separate line
    for argument in env::args() {
        println!("{argument}");
    }

    let v: Vec<String> = env::args().collect();
    println!("{}", v[0]);

    if v.len() <= 1 {
        return;
    }

    let s = &v[1];
    let i: i64 = s.parse().unwrap();
    println!("{}", i * 2);
}
