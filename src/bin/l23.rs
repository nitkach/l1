/// Develop a program to find the distance between two points,
/// which are represented as a Point structure with encapsulated parameters
/// x, y and a constructor.

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).powf(0.5)
    }
}

// can also be implemented via the `Add` trait.
// but I prefer to use methods, as this is a more explicit and clear approach.

// impl std::ops::Add<Point> for Point {
//     type Output = f64;

//     fn add(self, rhs: Point) -> Self::Output {
//         ((self.x - rhs.x).powi(2) + (self.y - rhs.y).powi(2)).powf(0.5)
//     }
// }

fn main() {
    let a = Point::new(2.0, -1.0);
    let b = Point::new(-3.0, 5.0);

    println!("Distance between a and b: {}", a.distance(&b));
}
