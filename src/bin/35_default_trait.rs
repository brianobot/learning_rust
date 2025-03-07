use std::default::Default;


#[allow(unused)]
#[derive(Default, Debug)]
struct SomeOption {
    foo: i32,
    bar: u32,
}

/*
Some Notes
- The derive attribute can be used to derive the Default trait if all the fields in the struct implements the default trait
- You can derive the Default trait for enum and make the default field with the #[default] attribute,
- the default field for an enum can not be non-unit or non-exhastive variant
- the Default trait is not dyn compatabile
*/


fn main() {
    let some_option: SomeOption = Default::default();
    println!("{:?}", some_option);
}