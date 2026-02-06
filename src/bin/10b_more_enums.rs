// An attribute to hide warnings for unused code.
#![allow(dead_code)]
#![allow(clippy::upper_case_acronyms)]

#[derive(Debug, Clone)]
enum Color {
    Hex(String),
    RGB(u8, u8, u8),
    HSL(u8, f32, f32),
}

fn inspect(color: Color) {
    match color {
        Color::Hex(_) => println!("Hex Color provided"),
        Color::RGB(_, _, _) => println!("RGB Color provided"),
        Color::HSL(_, _, _) => println!("HSL Color provided"),
    }
}

fn main() {
    let black = Color::Hex(String::from("000000"));
    let white = Color::RGB(20, 20, 20);

    dbg!(black.clone(), white.clone());

    inspect(black);
    inspect(white);

    // when using the Option Enum in rust
    // it is important to specify the normal expected value as a type when referencing the None variant
    // it is not important to do so for the Some variant, since the data type can be inferred from the value passed into Some

    let some_value = Some(0.21);
    let none: Option<f32> = None;

    // so the None type here is a type safe alternative to null values

    dbg!(some_value, none);

    // Enums can be type alias, weird but true
    type Vision = Color;

    let rainbow = Vision::HSL(12, 12.0, 23.0);
    dbg!(rainbow);

    type Word = String;

    let intro = Word::from("Hi!, My Name is Brian");
    println!("This is my Intro: {intro}");

    println!("==========================================");

    #[derive(Debug)]
    enum Planet {
        Mecury,
        Venus,
        Earth,
        Mars,
        Jupiter,
        Saturn,
        Neptune,
        Uranus,
        Unknown,
    }

    let lines = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

    let mut planet: Planet = Planet::Earth;

    for line in lines.lines() {
        let value: Vec<&str> = line.splitn(2, " ").collect();
        if value.len() == 1 {
            planet = Planet::Unknown;
        }

        match planet {
            Planet::Unknown => break,
            _ => continue,
        }
    }

    println!("planet = {:?}", planet);
}
