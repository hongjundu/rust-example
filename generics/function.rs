
use std::fmt;

#[derive(Debug)]
struct SGen<T> {
    value : T,
}

impl <T: fmt::Display> fmt::Display for SGen<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn foo<T: fmt::Display>(t : SGen<T>) {
    println!("{}", t)
}

fn main() {
    let a = SGen{value: 10};
    foo(a);

    let b = SGen{value: 3.3};
    foo(b);
}