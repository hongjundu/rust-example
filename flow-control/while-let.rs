fn main() {
    let mut opt = Some(0);

    while let Some(i) = opt {
        if i > 9 {
            println!("Greater than 9, quit!");
            opt = None;
        }else {
            println!("`i` is `{:?}`. Try again.", i);
            opt = Some(i + 1);
        }
    }

    println!("opt: {:?}", opt);
}