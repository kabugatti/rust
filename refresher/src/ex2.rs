// Exercise 2: Struct and Methods Practice

// Complete this struct and implement the required methods
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Constructor
    fn new(width: u32, height: u32) -> Rectangle {
        // Your implementation here
        Rectangle {
            width,
            height
        }
    }
    
    // Calculate area
    fn area(&self) -> u32 {
        // Your implementation here
        self.width * self.height
    }
    
    // Check if this rectangle can hold another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        // Your implementation here
        let p1 = 2 * (self.height + self.width);
        let p2 = 2 * (other.height + other.width);

        p1 > p2
    }
}

fn main() {
    let rect1 = Rectangle::new(23, 45);
    let rect2 = Rectangle::new(32, 50);

    println!("Width: {}, Height: {}", rect1.width, rect1.height);
    println!("Width: {}, Height: {}", rect2.width, rect2.height);

    let area1 = rect1.area();
    let area2 = rect2.area();

    println!("{}", area1);
    println!("{}", area2);

    println!("{}", rect1.can_hold(&rect2));
}