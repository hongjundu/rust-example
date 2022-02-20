
use std::mem;

fn print_slice(slice : &[i32]) {
    println!("first element: {}", slice[0]);
    println!("size of slice: {}", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 10000] = [0; 10000];

    println!("array occupies {} bytes", mem::size_of_val(&ys));
    print_slice(&xs);

    print_slice(&ys[0 .. 4])

}