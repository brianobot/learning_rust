use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();

    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(&table);
    // Shared References are Copy
    // Mutable References are not Copy
    dbg!(table);

    // Notice how eve when we passed a reference to the show function
    // we never reeally dereference it to get the value been referreed to
    // well that's because, the . operator implicity dereferences references when neccesary
    // and in this case, the println! macro triggered a dot operation somewhere to get this behaviour
    // the . operator can also implictly borrow a reference to it left operand when needed
    let mut v = vec![1, 2, 3, 4];
    v.sort(); // here the sort method needs a &mut to the object so .sort borrows mut v as &mut v
              // which is equivalent to (&mut v).sort()
              // this means that outside the . operator, you have to explicitly borrow with &
              // and dereference with *
              //
              // assigning to reference simply makes the reference to point to the new value
              // it does not affect the referent
              //
              // References to References
    struct Point(i32, i32);

    let point = Point(10, 20);
    let r = &point;
    let rr = &r;
    let rrr = &rr;

    // the . operator follows as many references as it need to find the target
    println!("Point x: {}", rrr.0);
    println!("Point y: {}", rrr.1);
    // this same behaviour of following the references until it target applies for comparison too
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx == rry);
    // if you want to check if two references point to the same memory
    // you can use the
    use std::ptr::eq;

    let result = eq(rrx, rry);
    println!("Result: {result}");
    // note that the operand for comparison must have the same type &T == &T, (not &&T == &T)
    //
    // References to slice are fat pointers, nothing special about them for now

    {
        let r;
        {
            let x = 1;
            r = &x; // at this point r is assigned and it valid
        } // x is dropped here and r is uninitialized here
          // assert_eq!(*r, 1); // tryin to access r here is a violation of everything Rust stands for
          // because the value bein referred to have been dropped at the end of the inner block
    }

    // lifetimes in functions signature helps rust access the relationship between the references passed into the function
    // and those returned from the function
    fn largest(v: &[i32]) -> &i32 {
        &v[v.len() - 1]
    }

    {
        let list = [1, 2, 3, 4, 5];
        let s = largest(&list);
        println!("S = {s}"); // this works
    }

    // structures containgin refereces
    struct S<'a> {
        r: &'a i32, // whenever a reference appears inside a structure, you must write out it's lifetime explicitly
    }

    let s;
    {
        let x = 10;
        s = S { r: &x }; // here a fresh S is created with a lifetime that lives for as long as s is valid
    }

    // assert_eq!(*s.r, 10);
    // when you get a shared reference to a value
    // the shared reference makes the referent read only, it basically locks down the referent
    let v = vec![1, 2, 3, 4, 5, 6];
    let r = &v;

    // provided you don't try to use the reference after the v has been moved, this code compiles
    let aside = v;
    println!("Aside: {aside:?}");
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("Works by {artist}");
        for work in works {
            println!("   {work}")
        }
    }
}
