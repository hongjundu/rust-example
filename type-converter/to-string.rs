use std::string::ToString;
use std::str::FromStr;
use std::num::ParseIntError;

struct Circle {
    radius: i32
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle radis: {}", self.radius)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let radius = s.parse::<i32>()?;
        Ok(Circle{radius})
    }
}

fn main() {
    let circle = Circle{radius: 6};
    println!("{}", circle.to_string());

    let circle2 : Circle = "100".parse().unwrap();
    println!("{}", circle2.to_string());
}