
fn main() {
    let closure = |i : i32| -> i32 { i + 1 };

    let a = 100;
    println!("{}", closure(a));
}