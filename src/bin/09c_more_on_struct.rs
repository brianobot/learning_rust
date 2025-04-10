// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Unit struct
struct Unit;

// Tuple struct
struct Pair(i32, i32);

// Struct with 2 fields
struct Point {
    x: u32,
    y: u32,
}

// structs can be resued as the field for other structs
struct Rectangle {
    bottom_left: Point,
    top_right: Point,
}

fn rect_area(rect: &Rectangle) -> u32 {
    let width = rect.top_right.x - rect.bottom_left.x;
    let height = rect.top_right.y - rect.bottom_left.y;
    width * height
}

fn square(point: Point, width: u32) -> Rectangle {
    let top_right = Point {
        x: point.x + width,
        y: point.y + width,
    };
    Rectangle {
        bottom_left: point,
        top_right: top_right,
    }
}

fn main() {
    // there are 3 types of structures that can be created in rust
    // - Tuple structs (typically named tuples)
    // - C like structs
    // - Unit Structs (without fields)

    let name = String::from("Brian");
    let age = 24; // updated this to reflect my current age,

    let brian = Person { name, age };

    dbg!(brian);

    let first_square = Rectangle {
        bottom_left: Point { x: 0, y: 0 },
        top_right: Point { x: 2, y: 2 },
    };

    let square_area = rect_area(&first_square);
    println!("Area of Square: {square_area}");

    let another_square = square(Point { x: 0, y: 0 }, 2);
    let another_square_area = rect_area(&another_square);

    println!("Area of another square: {another_square_area}");
}
