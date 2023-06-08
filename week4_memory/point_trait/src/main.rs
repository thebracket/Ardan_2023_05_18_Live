use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x, 
            y: self.y + rhs.y
        }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}

fn main() {
    let p = Point{ x: 1, y: 2};
    println!("{p:?}");

    let a = Point { x: 1, y: 2 };
    let b = Point { x: 3, y: 4 };
    let c = a + b;
    println!("c.x = {}, c.y = {}", c.x, c.y);
}