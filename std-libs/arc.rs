use std::sync::Arc;
use std::thread;

fn main() {

    let mut children = vec![];

    // 这个变量声明用来指定其值的地方。
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // 这里没有数值说明，因为它是一个指向内存堆中引用的指针。
        let apple = Arc::clone(&apple);

        children.push(thread::spawn(move || {
            // 由于使用了Arc，线程可以使用分配在 `Arc` 变量指针位置的值来生成。
            println!("{:?}", apple);
        }));
    }

    for child in children {
        // 等待线程结束。返回一个结果。
        let _ = child.join();
    }
}

