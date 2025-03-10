/*
this is the process of overloading the operators functionalities for some types in rust
examples of operators are add (+) sub (-) modulus (%) etc

operators that can be overloaded are available in the std::ops module
*/ 
#![allow(unused)]


use std::ops::{Add, Sub};

#[derive(Default, Debug, Clone)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first_name: String, last_name: String) -> Self {
        Person { first_name, last_name }
    }
}

impl Add for Person {
    type Output = Marriage;

    fn add(self, rhs: Self) -> Self::Output {
        Marriage {
            husband: self,
            wife: rhs,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Marriage {
    husband: Person,
    wife: Person,
    location: String,
    date: chrono::NaiveDate,
}

#[derive(Debug, Default, Clone)]
struct Divorce {
    marriage: Marriage,
    initiator: Person,
}

impl Sub<Person> for Marriage {
    type Output = Divorce;
    
    fn sub(self, rhs: Person) -> Self::Output {
        Divorce {
            marriage: self.clone(),
            initiator: rhs.clone(),
        }
    }
}



fn main() {
    let peter = Person::new("Peter".to_string(), "Okon".to_string());
    let sarah = Person::new("Sarah".to_string(), "Udoh".to_string());

    let marriage = peter + sarah.clone();
    println!("Marriage = {:?}", marriage);

    let divorce = marriage - sarah;
    println!("Divorce = {:?}", divorce);
}