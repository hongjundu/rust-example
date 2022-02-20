
use std::convert::From;

#[derive(Debug)]
#[allow(dead_code)]
struct Number {
    value : i32,
}

impl From<i32> for Number {
    fn from(value : i32) -> Self {
        Number{ value }
    }
}

fn main () {
    let num = Number::from(20);
    println!("number: {:?}", num);

    let i = 20i32;
    let num2 : Number = i.into();
    println!("number2: {:?}", num);
}