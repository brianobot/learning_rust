#![allow(unused)]


#[derive(Debug)]
struct Point<T: Clone, U: Clone> { // the generic type is defined here, Notice the use of <> to denote definition
    x: T, // it is used here, 
    y: U,
}

impl<T: Clone, U: Clone> Point<T, U> { // the type on the impl keyword is needed to let the compiler know
    // what type of generic you are referring to, basically we are also defintion the generic type
    // for the impl block

    // if however you are implementing the impl block for only a specific type, you do not need
    // to define the type for the impl block

    // impl for Point<i32> {}  this works
    pub fn x(&self) -> &T {
        &self.x
    }

    // take a minute or more and appreciate this art here
    pub fn mixup<T2: Clone, U2: Clone>(&self, other: &Point<T2, U2>) -> Point<T, U2> {
        // notice how mixing up two point does not destroy the original individial points 
        Point {
            x: self.x.clone(),
            y: other.y.clone(),
        }
    }
}

trait Pointable<T: Clone, U: Clone> {
    fn get_distance(&self) -> &T;
}

impl<T: Clone, U: Clone> Pointable<T, U> for Point<T, U> {
    fn get_distance(&self) -> &T {
        &self.x
    }
}


fn main() {
    let int_point = Point { x: 5, y: 7 };
    let float_point = Point { x: 2.3, y: 3.56 };
    let mixed_point = Point { x: 12, y: 4.3 };

    println!("X = {}", mixed_point.x());

    let first_point = Point { x: 10, y: 10 };
    let second_point = Point { x: 20.0, y: 20.0 };

    let third_point = first_point.mixup(&second_point);
    
    println!("First Point: {:?}", first_point);
    println!("Second Point: {:?}", second_point);
    println!("Third Point: {:?}", third_point);

    let distance = calculate_distance(&first_point);
    println!("First point distance = {distance}");

    let another_distance = calculate_distance_2(&second_point);
    println!("Second point distance: {another_distance}");
    
}


// we can use trait as method/function parameters
fn calculate_distance<T: Clone, U: Clone>(point: &impl Pointable<T, U>) -> &T {
    point.get_distance()
}

fn calculate_distance_2<T, U, P>(point: &P) -> &T 
    where T: Clone,
          U: Clone,
          P: Pointable<T, U> 
    {
        point.get_distance()
    }