
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn original() -> Point {
        Point{ x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point{ x: x, y: y }
    }

    fn test(&self) -> f64 {
        self.x * self.y
    }
}

fn main() {
    let point = Point::original();
    println!("point original: {:?}",point);

    let point = Point::new(10f64,20f64);
    println!("point: {:?}",point);
    println!("point test: {}",point.test());
}