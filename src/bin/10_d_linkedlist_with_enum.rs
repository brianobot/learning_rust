use crate::List::*;

#[allow(dead_code)]
enum List {
    Cons(u32, Box<List>),
    Nil,
}

#[allow(dead_code)]
impl List {
    fn new() -> List {
        Nil
    }

    // research more to understand the Box data structure
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Nil => "Nil".to_string(),
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(10);
    list = list.prepend(20);
    list = list.prepend(30);
    list = list.prepend(40);
    list = list.prepend(50);
    list = list.prepend(60);

    println!("List = {}", list.stringify());
}
