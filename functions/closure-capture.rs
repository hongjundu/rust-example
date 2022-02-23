
fn main() {
    let color = "red";
    let print = || { println!("color: {}", color); };
    print();

    let mut count = 0;
    let mut inc = || { count+=1;};
    inc();
    inc();

    println!("count: {}", count);
    
    let reborrow = &mut count;
    println!("reborrow: {}", reborrow);

    let movable = Box::new(0);
    let consume = || {std::mem::drop(movable);};
    consume();
    //consume();

    let vec = vec![1,2,3,4];
    let contains = |v : &i32| -> bool { vec.contains(v) };
    println!("{}", contains(&4));
}