
use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64,i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let s = Structure(10);
    println!("{}", s);

    let mm = MinMax(0, 10);
    println!("Debug for MinMax: {:?}", mm);
    println!("Display for MinMax: {}", mm);

    let pt = Point2D{x: 1f64, y: 2f64};
    println!("Debug for Point2D: {:?}", pt);
    println!("Display for Point2D: {}", pt);
}
