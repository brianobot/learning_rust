fn main() {
    // enums give a way of saying that a value is one of a possible set of values

    // this example shows a data that can be either a V4 or a V6
    // as can be seen from the lack of semi-colon, the enum is an expression
    #[derive(Debug)]
    enum IPAddrKind {
        // the syntax uses to defined variants resemble the struct
        V4, 
        V6,
    }

    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;

    // the variants of an enum are namespaced under it's identifier, hence we use the double colon
    // both four and six are of the same type and any function that takes the enum
    // IPAddrKind would also take four and six

    fn route(ip_kind: IPAddrKind) {
        println!("ip_kind: {:?}", ip_kind);
    }

    route(four);
    route(six);
    // the value of an enum contain information about which variant it is
    // 

    enum _IPAddrKindV2 {
        V4(String),
        V6(String),
    }
    // we can attache data to each variant directly as shown above so there is no need to
    // make the variants a struct, anothere important thing here is that
    // the Name of the variant becomes a function call that constructs an instance of that variant
    // in this case V4, and V6 are function calls that takes in String arguments and return an instances of IpAddrKindV1 type

    // another advantage with Enums is that each variants can have very different data types
    #[derive(Debug)]
    #[allow(dead_code)] // this allows for dead code to be compiled
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let _localhost = IpAddr::V4(192, 31, 23, 42);
    let _loopback = IpAddr::V6(String::from("::1"));

    // println!("localhost: {:#?}", localhost);
    // println!("holoopbackme: {:#?}", loopback);

    // ny kind of data can be placed inside the enum variants, any data type, you can even include another enum
    // we can also define methods on enums too just like we can on structs

    impl IpAddr {
        fn call(&self) -> u8 {
            10
        }
    }

    // call the method from the enum variant
    let result = _localhost.call();

    println!("Result value: {result}");

    // the Option enum from the standard library
    // the option enums encodes the very common scenario where a value could be something or nothing

    enum _Option<T> {
        None,
        Some(T),
    } 

    // the option enum and its variants are included in the prelude so you do not have to import them before using them 
    // in your code, the code about uses T which is what is called a generic datatype
    // and it goes to say that what ever the value of T is is the data type of the Option instance

    // so let some_number = Some(1) // means some_number is of the option<i32> datatype
    // this means we can not move with the assumption and expectation tht rust would treat this type
    // as the normal i32 data type, so in other to use a variant of Option that is of the type
    // Option<i32> we have to convert that to a i32 type first

    // Enums can also used as c like enums by explicitly providing the discriminators
    #[derive(Debug)]
    enum Color {
        Red = 0x12345431,
        Blue = 0x2345531,
        Green = 0x12375431,
    }

    println!("Roses are {:?}", Color::Red);
    println!("Violets are {:?}", Color::Blue);
    println!("Leaves are {:?}", Color::Green);

}