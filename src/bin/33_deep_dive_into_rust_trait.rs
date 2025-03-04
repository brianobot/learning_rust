// a trait is a way to define shared behaviour, it is essentially an interface that defines
// expected methods with their signatures, thereby enabling polymorphism and interface abstraction

// trait defintion
#[allow(unused)]
trait Printable {
    fn print(&self);
}


#[derive(Debug)]
#[allow(unused)]
struct Paper {}


// trait implementation
impl Printable for Paper {
    fn print(&self) {
        println!("{:?}", self);
    }
}


impl Printable for i32 {
    fn print(&self) {
        println!("{:?}", self);
    }
}

// trait inheritance
#[allow(unused)]
trait PrintableWithLabel: Printable {
    fn print_with_label(&self, label: &str) {
        print!("{}: ", label);
        self.print();
    }
}



impl PrintableWithLabel for i32 {}

fn main() {
    let number: i32 = 23;
    number.print_with_label("Age");
}

#[allow(unused)]
fn print_static<T: Printable>(value: T) {
    value.print();
}

#[allow(unused)]
fn print_dynamic(value: &dyn Printable) {
    value.print();
}



//  print_static uses a generic parameter T, which must implement the Printable trait. When this function is called, the compiler generates separate code for each type that is passed to it (static dispatch).
// print_dynamic uses a trait object (&dyn Printable) as a parameter. This enables dynamic dispatch, allowing the function to process any type implementing the Printable trait.