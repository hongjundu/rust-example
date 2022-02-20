

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person{ name, age};
    println!("{:?}", peter);

    let Person{name: n, age: a} = peter;
    println!("name: {}", n);
    println!("age: {}", a);
}