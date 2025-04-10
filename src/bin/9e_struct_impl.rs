#![allow(dead_code)]

use std::cell::Cell;
use std::fmt;
use std::fmt::Display;

enum Character {
    Mage,
    Archer,
    Warrior,
}

impl Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Character::Mage => write!(f, "Mage"),
            Character::Archer => write!(f, "Archer"),
            Character::Warrior => write!(f, "Warrior"),
        }
    }
}

#[derive(Debug)]
struct VehicleTuple(String, String, u32);

struct Player {
    name: String,
    hp: i32,
    mp: i32,
    class: Character,
}

// we can control the indivial mutability of fields on a struct with the Cell and RefCell types
// Cell and RefCell are types that provide interior mutability, meaning that the data can be mutated even when the type is immutable
// Cell is used for types that implement the Copy trait, while RefCell is used for types that do not implement the Copy trait
// Cell and RefCell are used to provide mutability in a single-threaded context

#[derive(Debug)]
struct SecurePlayer<'a> {
    username: Cell<&'a str>,
    level: u32,
    hp: u32,
}

fn main() {
    let one = Character::Archer;
    let vehicle = VehicleTuple("Toyota".to_string(), "Corolla".to_string(), 2019);
    println!("One = {}", one);
    println!("Vehicle = {:?}", vehicle);

    let secure_player = SecurePlayer {
        username: Cell::new("John"),
        level: 1,
        hp: 100,
    };

    // notice how we mutate the field value here even when the entire struct is immutable
    secure_player.username.set("Jane");
    let first_name = secure_player.username.get();
    println!("Secure Player = {:?}", secure_player);
    println!("Secure player first name = {}", first_name);
}
