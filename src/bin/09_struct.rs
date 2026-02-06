// the struct keyword is an expression and not a statement
// this means that we do not have to end it with a semi-colon
#[derive(Debug)]
// structs are similar to data classes in python
struct User {
    // the values in a struct are called its field
    // we can not have partial field mutablity in struct,
    username: String, // notice that our struct fields get String data, which means the own the value
    email: String, // it is possible to use &str as fields in struct, but means the use of lifetimes, we will see it later
    sign_in_count: u64,
    active: bool,
}

// methods can be attached to a struct
// this is called an implemetation block and a struct can have more that 1
impl User {
    // methods are similar to methods in Python
    // methods are called with dot notation
    fn greet(&self) -> String {
        String::from("hello world")
    }

    // methods can take mutable reference to the parent struct, but is more often to pass a refernce instead
    fn has_same_username(&self, user: &User) -> bool {
        self.username == user.username
    }
}

// let use another implementation block to keep the associated functions
// this implementation block have been placed inside the first, but this is for the sake of example
impl User {
    // we do not pass in the self argument here becaise it is an associated function
    // associated functions are called with struct::associated_function_name
    fn create_user(email: String, username: String) -> User {
        User {
            email,
            username,
            sign_in_count: 1,
            active: true,
        }
    }
}

// we can also create structs without name fields,
// these are called tuple strutcts, example are shown below
// tuple structs are statements, which is why they are terminates with semi-colon
// to make this struct printable we can implement the Display trait or add this line
#[derive(Debug)]
struct Color(i32, i32, i32);
// tuple structs are similar to tuples, in that they can destructured just like regular tuples
// and you can use a dot notation followed by the index of the field
struct _Point(f64, f64, f64); // i used the userscore infront, because rust would not allow me to compile
// if i am not using the value defined, this way it ignores that error when compiling

// you can also defined unit like struct without any fields
// so this are terminated with semi-colon unlike normal struct expressions
#[derive(Debug)]
struct AlwaysTrue;

fn main() {
    // A struct, or structure, is a custom data type that lets you package together and name multiple related values that make
    // up a meaningful group, this is similar to an OOP object's data attributes

    // the syntax for the struct  is similar to the dictionary in python ecosystem
    // when a new struct in created like,, it is called a new instance of the struct
    let user1 = User {
        username: String::from("dev_brian"),
        email: String::from("brianobot9@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    // we can access values from the struct using dot notation
    let (username, _email, _active, _sign_in_count) = (
        user1.username,
        user1.email,
        user1.active,
        user1.sign_in_count,
    );
    println!("User1 Username: {username}");
    // struct are immutable by default to
    // in order to change the value of a field from outside the struct instantiation, you must set it as mut
    let mut user_2 = User {
        username: String::from("dev_brian"),
        email: String::from("brianobot9@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    user_2.username = String::from("new_username");
    // while compiling this code the first time
    // i noticed that rust does not allow for field access directly in {} in the println macro,
    // but it does allow for those field access to be ised as positional argument instead
    println!("User2 Username: {}", user_2.username);

    let user3 = build_user("ThirdUser".to_string(), "safe@gmail.com".to_string());
    println!("User3 Username: {}", user3.username);

    // we can also create a new struct from the fields of an exisiting struct like so
    // this is similar to the spread operation in some languages
    let user4 = User {
        email: "test@email.com".to_string(),
        // when using this spread operator here, it must come last
        ..user3
    };

    // trying to access the user3 struct as a whole after accessing it fields to user4 fails
    // this is because, the String type does not implement Copy and is moved to user4 Username field
    // this makes user3.username invalid
    // println!("Try to access user3 here: {:?}", user3);

    println!("User4 Username: {}", user4.username);
    println!("User4 Greetings: {}", user4.greet());
    println!(
        "User4 Has same username as User2 {}",
        user4.has_same_username(&user_2)
    );

    // let use the associated function here
    let _user5 = User::create_user(String::from("email"), String::from("username"));

    println!("User5: {:#?}", _user5);

    // instantiate the color struct here
    let red = Color(10, 20, 20);
    println!("Red: {:#?}", red);
    println!("Red [0]: {:?}", red.0);
    println!("Red [1]: {:?}", red.1);
    println!("Red [2]: {:?}", red.2);

    let valid = AlwaysTrue;
    println!("Valid: {:?}", valid);
}

fn build_user(username: String, email: String) -> User {
    User {
        // the order of the struct fields do not matter, provided they are all accounted for
        // and because our argument have the same name as the field names in our struct
        // we can simplify this by using a field init shorthand syntax like
        // email, instead of email: email,
        // username, instead of username: username,
        email,
        username,
        sign_in_count: 1,
        active: true,
    }

    // NOTE:
    // by default member fields in structs are private
    // which means that even if the structs are marked as public,
    // their fields are still hidden unless explicityl marked as public
    // they can still be accessed from other codes within the same crate as the struct
    // but to outside access they are hidden by default
}
