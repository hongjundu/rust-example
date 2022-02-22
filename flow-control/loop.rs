fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过这次迭代的剩下内容
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // 退出循环
            break;
        }
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("rusult: {}", result);

    // for i in 1..=100 {
    //     println!("{}",i);
    // }

    let names = vec!["Tom", "Jack", "Jerry"];
    for name in names.into_iter() {
        match name {
            _ => println!("Hello {}", name),
        }
    }



    //println!("names: {:?}", names);
}
