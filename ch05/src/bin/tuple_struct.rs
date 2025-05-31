use std::fmt;

struct Color(i32, i32, i32);

// Implementation assumes that the `Color` struct models an RGB color
impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Color")
            .field("red", &self.0)
            .field("green", &self.1)
            .field("blue", &self.2)
            .finish()
    }
}

struct Point(i32, i32, i32);

// Implementation assumes that the `Point` struct uses Cartesian coordinates
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.0)
            .field("y", &self.1)
            .field("z", &self.2)
            .finish()
    }
}

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("black = {:?}", black);
    println!("origin = {:?}", origin);
}