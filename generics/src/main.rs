struct Enum(i32);
trait Interface1 {
    fn action1(&self) -> i32;
}
impl Interface1 for Enum {
    fn action1(&self) -> i32 {
        self.0
    }
}
trait Interface2 {
    fn action2(&self) -> i32;
}
impl<T> Interface2 for T
where
    T: Interface1,
{
    fn action2(&self) -> i32 {
        self.action1() * 2
    }
}

fn main() {
    let a = Enum(21);
    let b = a.action2();
    println!("{}", b);
}
