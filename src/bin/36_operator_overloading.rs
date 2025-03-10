/*
this is the process of overloading the operators functionalities for some types in rust
examples of operators are add (+) sub (-) modulus (%) etc

operators that can be overloaded are available in the std::ops module

+ is a syntactic sugar for the add method
1u32 + 2u32 === 1u32.add(2u32)
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

// supporting Marriage - Person == Divorce
impl Sub<Person> for Marriage {
    type Output = Divorce;
    
    fn sub(self, rhs: Person) -> Self::Output {
        Divorce {
            marriage: self.clone(),
            initiator: rhs.clone(),
        }
    }
}

impl Add<i32> for Marriage {
    type Output = Marriage;
    
    // it's important to note that the trait definition doesn't
    // have to match the trait method argument names, but the types must match
    fn add(self, other: i32) -> Self::Output {
        self
    }

    
}


fn main() {
    let peter = Person::new("Peter".to_string(), "Okon".to_string());
    let sarah = Person::new("Sarah".to_string(), "Udoh".to_string());

    let marriage = peter + sarah.clone();
    println!("Marriage = {:?}", marriage);

    let divorce = marriage.clone() - sarah;
    println!("Divorce = {:?}", divorce);

    let marrage_upgrade = marriage + 1i32;
    println!("Marriage Upgrade = {:?}", marrage_upgrade);
}