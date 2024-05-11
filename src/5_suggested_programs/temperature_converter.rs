use std::io;

fn main() {
    // introduce the program
    println!("Temperature Converter");
    println!("=======================");

    // unit would store the unit selected by the user as base for conversion
    let mut unit = String::new();

    println!("PLEASE ENTER BASE UNIT: (c / f)");

    io::stdin()
        .read_line(&mut unit)
        .expect("Could not read the value from the user");

    let unit = unit.trim();

    let converter = match unit {
        "c" => celsius_to_fahr,
        "f" => fahr_to_celsius,
        _ => celsius_to_fahr,
    };

    // get base value from the user
    let mut base_value = String::new();

    io::stdin()
        .read_line(&mut base_value)
        .expect("Could not read the value from the user");

    let base_value: f64 = base_value
        .trim()
        .parse()
        .expect("Can not Convert {base_value} to float");

    println!("Base Value: {base_value}");

    // call the conversion function for the selected unit
    let result = converter(base_value);
    let unit = if unit == "c" { "°C" } else { "F" };
    println!("Result: 0.0 = {result}{unit}");

}


// convert temperature value from celsius unit to fahrenheit
fn celsius_to_fahr(x: f64) -> f64 {
    // notice the efforts to represent all the variables in the equations as floating point
    (x * (9.0 / 5.0)) + 32.0
}


// convert temperature unit from fahrenheit to celsius
fn fahr_to_celsius(y: f64) -> f64 {
    // notice the efforts to represent all the variables in the equations as floating point
    (y - 32.0) * (5.0 / 9.0)
}