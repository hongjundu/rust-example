
// demostrate Debug

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let s = Structure(1);
    println!("I am a {:?}", s);

    let d = Deep(Structure(2));
    println!("I am a {:?}", d);

    let name = "xixi";
    let age = 8;
    let p = Person{ name, age };
    println!("I am {:?}", p);
    println!("I am {:#?}", p);
}