
fn main() {

    // destructing and dereference
   let ref_int = &4;

   match ref_int {
       &val => println!("Got a value via destructing: {}", val),
   }

   match *ref_int {
       val => println!("Got a value via dereference: {}", val),
   }

   let value = 10;
   match value {
       ref ref_value => println!("reference: {}", *ref_value),
   }

   let mut mut_value = 5;
   match mut_value {
       ref mut ref_mut_value => *ref_mut_value += 100,
   }
   println!("mut_value: {}", mut_value);

   // desctruct struct

   let foo = Foo { x: 10, y: 20};
   println!("foo: {:?}", foo);

   let Foo {x, y} = foo;
   println!("x = {}, y = {}", x,y);


   // guard expression
   let pair = (1,-1);
   match pair {
       (x, y) if x == y => println!("x == y"),
       (x, y) if x + y == 0 => println!("0"),
       (x, y) if x % 2 == 2 => println!("the first one is odd"),
       _ => println!("others"),
   }

   // re-binding

   fn age() -> i32 {
       45
   }

   match age() {
       0 => println!("I am not born yet"),
       n @ 1..=12 => println!("I am a child of {}", n),
       n @ 13..=19 => println!("I am a teen of {}", n),
       n => println!("I am an old persion of {}",n),
   }

   fn some_number() -> Option<i32> {
       Some(42)
   }

   match some_number() {
       Some(n @ 42) => println!("The answer: {}", n),
       Some(n) => println!("Not the answer: {}", n),
       _ => println!("error"),
   }
}

#[derive(Debug)]
struct Foo {
    x : i32,
    y : i32,
}