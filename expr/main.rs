fn main() {
    let b = false;

    let x = {
        if b  { 1 }
        else { 0 }
    };

    println!("x = {}", x);
}