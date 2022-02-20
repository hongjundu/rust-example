
#![allow(dead_code)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x: i64, y: i64}
}

enum Number {
    Zero,
    One,
    Two,
}

// 拥有显式辨别值（explicit discriminator）的 enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page Loaded"),
        WebEvent::PageUnload => println!("Page Unloaded"),
        WebEvent::KeyPress(c) => println!("Key Pressed: {}", c),
        WebEvent::Paste(str) => println!("Pasted: {}", str),
        WebEvent::Click{x,y} => {
            println!("Clicked: x: {}, y: {}",x, y)
        },
    }
}

fn main () {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}