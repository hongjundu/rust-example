
fn reverse(pair: (i32,bool)) -> (bool, i32) {
    let (i, b) = pair;
    (b,i)
}

fn main() {
    let pair = (10, true);
    println!("pair: {:?}", pair);

    let pair = reverse(pair);
    println!("pair: {:?}", pair);

}