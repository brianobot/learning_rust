struct Rectangle {
    width: u32,
    height: u32,
}

// implementation block for the Rectangle struct
// everything here is associated to the Rectangle struct
impl Rectangle {
    // &self is a shorthand for self: &Self
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
