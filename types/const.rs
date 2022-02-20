
static LANGUAGE : &'static str = "RUST";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
   
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is big: {}", 5, is_big(5));
    println!("{} is big: {}", 11, is_big(11));
}