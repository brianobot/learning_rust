use std::{collections::HashMap, thread};

fn main() -> std::io::Result<()> {
    let mut integers = vec![4, 6, 3, 6, 3, 6, 2, 7, 3, 8, 4, 23, 56];
    integers.sort();

    println!("Sorted Integers: {integers:?}");

    #[derive(Default, Debug)]
    #[allow(dead_code)]
    struct City {
        name: String,
        capital: String,
        population: i64,
    }

    impl City {
        fn get_statistic(&self, _stat: Statistic) -> i64 {
            10i64
        }
    }

    #[derive(Copy, Clone)]
    struct Statistic {}

    fn start_sorting_thread(
        mut cities: Vec<City>,
        stat: Statistic,
    ) -> thread::JoinHandle<Vec<City>> {
        // the move infront of the closure tells the rust compiler that the closure doesn't borrow the values it uses
        // but it steals theme
        // this first closure takes ownership of stat
        let key_fn = move |city: &City| -> i64 { -city.get_statistic(stat) };

        // the second closure here, takes ownership of cities and the key_fn closure
        thread::spawn(move || {
            cities.sort_by_key(key_fn);
            cities
        })
    }

    let cities = vec![
        City {
            name: "Uyo".to_string(),
            ..Default::default()
        },
        City {
            name: "Kano".to_string(),
            ..Default::default()
        },
    ];
    let statistic = Statistic {};
    let result = start_sorting_thread(cities, statistic).join().unwrap();

    dbg!(result);
    // Rust offer 2 ways for closures to take data from the enclosing environement, moves
    //
    // functions and closures have types just like regular values too
    // structs may have function typed fields
    // a vector can store functions provided their types are the same
    // a function value size is just the size of the machine code since they are pointers
    // Note: Closures do not have the same type as functions

    fn calculate(x: i32) -> i32 {
        x * 2
    }

    // this is the type of the calculate function above
    let _func: fn(i32) -> i32 = calculate;

    // Every Closure has a type that's only known to the compiler
    // and no two closures have the same type
    // so when working with closures, your code should be generic and filter based on the Fn(T) trait which
    // functions and closures implement
    //
    // closures are very very fast, they are faster than functions pointer

    // FnOnce are closures that kill the values they take and hence must only be called onces
    // FnMut are closures that mutate the values they take and are not safe to be passed to threads, this do not kill (drop) the values
    //
    // Closures are represented as struct that holds references to the values the contain or the values the contain
    // based on whether they are borrow closures or move closures

    // Closures can also be Copy or Moves types based on how they used the values the capture
    // if everything a borrow closure capture is Copy, the closure is Copy too
    // if everything a move closure capture is Copy, the closure is Copy too, this applies for Clone types too
    //
    let mut greeting = String::from("Hello, ");
    let greet = move |name| {
        greeting.push_str(name);
        println!("{}", greeting);
    };
    greet.clone()("Alfred"); // this clones the value in the greeting for each call to clone
    greet.clone()("Bruce");

    // implementation of a router as seen in actix-web to show how closures can be used for callbacks
    #[derive(Default)]
    #[allow(dead_code)]
    struct Request {
        method: String,
        url: String,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    }

    #[derive(Default)]
    #[allow(dead_code)]
    struct Response {
        code: u32,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    }

    type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

    struct BasicRouter {
        routes: HashMap<String, BoxedCallback>,
    }

    impl BasicRouter {
        fn new() -> Self {
            Self {
                routes: HashMap::new(),
            }
        }

        fn add_route<C>(mut self, url: &str, callback: C) -> Self
        where
            C: Fn(&Request) -> Response + 'static,
        {
            self.routes.insert(url.to_string(), Box::new(callback));
            self
        }

        fn not_found_response(&self) -> Response {
            Response::default()
        }

        fn handle_request(&self, req: &Request) -> Response {
            match self.routes.get(&req.url) {
                Some(callback) => callback(req),
                None => self.not_found_response(),
            }
        }
    }

    struct FnPointerRouter {
        routes: HashMap<String, fn(&Request) -> Response>,
    }

    impl FnPointerRouter {
        fn new() -> Self {
            Self {
                routes: HashMap::new(),
            }
        }

        fn add_route(mut self, url: &str, callback: fn(&Request) -> Response) -> Self {
            self.routes.insert(url.to_string(), callback);
            self
        }

        fn not_found_response(&self) -> Response {
            Response::default()
        }

        fn handle_request(&self, req: &Request) -> Response {
            match self.routes.get(&req.url) {
                Some(callback) => callback(req),
                None => self.not_found_response(),
            }
        }
    }

    fn home(_req: &Request) -> Response {
        println!("Running Home Route Handler ✅");
        Response::default()
    }

    fn about(_req: &Request) -> Response {
        println!("Running About Route Handler ✅");
        Response::default()
    }

    let router = BasicRouter::new()
        .add_route("/", home)
        .add_route("/about", about);

    let fn_pointer_router = FnPointerRouter::new()
        .add_route("/", home)
        .add_route("/home", home)
        .add_route("/about", about);

    println!("Creating Requests");
    let home_request = Request {
        url: "/".to_string(),
        ..Default::default()
    };
    let about_request = Request {
        url: "/about".to_string(),
        ..Default::default()
    };

    println!("Successfully Created Requests");
    let _response = router.handle_request(&home_request);
    let _response = router.handle_request(&about_request);

    let _response = fn_pointer_router.handle_request(&home_request);
    let _response = fn_pointer_router.handle_request(&about_request);

    Ok(())
}
