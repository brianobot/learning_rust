// closures: anonymous functions that can capture their environment

#[derive(Debug, Clone, Copy, PartialEq,)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }



    }
}


fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    fn add_one_v1(x: i32) -> i32 { x + 1}
    let add_one_v2 = |x: i32| -> i32 { x + 1};
    let add_one_v3 = |x| { x + 1};
    let add_one_v4 = |x| x + 1;

    println!("Add One v1 = {}", add_one_v1(1));
    println!("Add One v2 = {}", add_one_v2(1));
    println!("Add One v3 = {}", add_one_v3(1));
    println!("Add One v4 = {}", add_one_v4(1));

    // closures take arguments in one of three ways
    /*
    1- Immutable reference
    2- Mutable refernece
    3- by Value
     */

    let echo = |x: &String| println!("Echoing {}", x);
    
    echo(&"Brian".to_owned());

}

// closures are LIKE anonymous functions, they do no require you to annotate the type
// of the arguments or the return type of the value like functions do

// closures are usually used in a smaller context that functions and from here
// and the compiler can infer the types of the parameters like it can from certain
// variables declarations,but as with variables we can add type annotation at the cost
// of being more verbose than is strictly necessary

// the compiler infers the type of the parameters for the closures on it first usage and further usage must
// comply with the inferred type needed
