trait Conversion<T> {
    fn convert(&self) -> T;
}

trait Conversion2 {
    type Item;

    fn convert(&self) -> Self::Item;
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Person {
    name: String,
}

impl Person {
    fn new(name: &str) -> Self {
        Person {
            name: name.to_string(),
        }
    }
}

impl Conversion<Person> for Person {
    fn convert(&self) -> Person {
        self.clone()
    }
}

impl Conversion2 for Person {
    type Item = Person;

    fn convert(&self) -> Self::Item {
        self.clone()
    }
}

fn main() {
    let person = Person::new("Brian");

    let new_person = Conversion::convert(&person);
    println!("New Person: {:?}", new_person);

    let newer_person = <Person as Conversion2>::convert(&new_person);
    println!("Newer Person: {:?}", newer_person);
}
