use std::time::{Duration, Instant};

fn main() {
    // Suppose you are building a chat server, there are alot of things to manage
    // incoming packets, group subscriptions, outing going packets etc
    // ideally you could just create a new thread for each incoming connection
    //
    // use std::{net, thread};

    // let listener = net::TcpListener::bind(address)?;

    // for socket_result in listener.incoming() {
    //     let socket = socket_result?;

    //     let groups = chat_group_table.clone();
    //     thread::spawn(|| {
    //         log_error(serve(socket, groups));
    //     });
    // }
    //
    // this works well until scale becomes an issue
    //

    // A future is a type that implements the Future trait
    #[allow(unused_imports)]
    use std::future::Future;

    // the async version of any function uses the same argument as the synchronouse version, only the return type varies
    // calling a asynchronous function do not actually do any computation outside constructing and returning the Future for that function
    // The future returned from an async function must hold all the datta needed to process the actual function body
    //
    // the async-std crate provides async version for all the standard IO facilities
    // a future returned by an async function wrapps all the data needed to run the function body
    // when an async function is called, the type of the Future is generated automatically by the compiler
    // and this type doesn't have a name, all we know about it is that it's implements the Future<Output=R> trait
    // where R is the actual output on the function
    //
    // an await expressoni takes ownership of the future and polls it
    // since await expressions depends on the ability to pause and resume functions
    // they can only be used within async functions
    //
    // in order to call an async from from inside a sync function, you can the the async_std::task::block_on function
    // the block_on function is a synchronous function that blocks until the async funnction returns a value
    // because ot blocks it should never be used in an async function
    //
    #[allow(dead_code)]
    async fn count_to_ten() -> usize {
        let mut value: usize = 0;
        for i in 0..10_000 {
            value = i;
            println!("{i}");
        }

        value
    }

    use async_std;

    // let _future = count_to_ten(); // in this case the actual logic of the function is not ran
    // let result = async_std::task::block_on(count_to_ten());
    // println!("Result: {result}");

    // Send trait allows types that implement it to be passed over to other threads or concurrent computation
    // Sync trait allow types that implement it to share data between concurrent parts of a program

    // the future has nothing to do with how it is ran, that's the Job of the executor whose job is running (polling) the
    // future appropriately in order to get the Future's final results from the future

    // it does sound simplier writing a for loop to continously poll a future until it returns a Ready enum
    // but this effectively wastes CPU cycles but block_on function knows when best to poll the future and when to go to sleep
    // to better use the CPU cycles effectively
    //
    // but simply blocking  thread with async_std::task::block_on is no better than running the synchronous version of our functions
    // to get the benefits of async programming and do other jumps while a thread is being blocked we use the
    // async_std::task::spawn_local

    async fn cheapo_request(host: String, port: u16, path: String) -> std::io::Result<String> {
        eprintln!("Running Cheapo Request for {host}:{port}/{path}");
        async_std::task::sleep(Duration::from_secs(2)).await;
        Ok(String::from("Successful"))
    }

    async fn many_requests(requests: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
        let mut handles = vec![];
        for (host, port, path) in requests {
            handles.push(async_std::task::spawn_local(cheapo_request(
                host, port, path,
            )))
        }

        let mut responses = vec![];
        for handle in handles {
            responses.push(handle.await);
        }

        responses
    }

    let requests = vec![
        ("homelander".to_string(), 9090u16, "home".to_string()),
        ("planet".to_string(), 8000, "about".to_string()),
        ("movies".to_string(), 8090, "list".to_string()),
    ];

    let start_time = Instant::now();
    let results = async_std::task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(response) => println!("{}", response),
            Err(err) => eprintln!("error: {}", err),
        }
    }

    println!("Elapsed Time: {:?}", start_time.elapsed());

    // Async blocks returns a future of the last expression of the block
    // you can use await expression inside an async block
    // async blocks can captures variables from their surrounding like closures and they can steal those variables
    // by prefixing the block with move, async move { ... }
    //
    // async_std::task::spawn is just like spawn_local but it starts executing the future immediately in a pool of thread workers
    // and it only accepts futures that implement the Send trait since it moves the futures across threads
    // a future is Send only if all the values it contains is Send, all function arguments, local variables and even anynomous temporary values must be Send
    //
    // if you're using a long running computation in an async context, it's a nice work around
    // to call async_std::task::yield_now().await frequently within your long computation in order
    // to ensure your future does not starve other futures (tasks) from running, what this does is effect yield
    // control to other tasks and immediately makes your task as being ready to continue again
    //
    // in cases where you can't directly control the codes for the long running computation, such as using a library code
    // or C or C++ code, you can use
    // async_std::task::spawn_blocking, this takes a closure and returns a Future over the return value of the closures
    // it runs it on it's own thread
    //
    async fn many_requests_v2(urls: &[String]) -> Vec<Result<String, surf::Error>> {
        let client = surf::Client::new();

        let mut handles = vec![];
        for url in urls {
            #[allow(clippy::needless_borrows_for_generic_args)]
            let request = client.get(&url).recv_string();
            handles.push(async_std::task::spawn(request));
        }

        let mut results = vec![];
        for handle in handles {
            results.push(handle.await);
        }

        results
    }

    let requests = &[
        "http://example.com".to_string(),
        "https://www.red-bean.com".to_string(),
        "https://en.wikipedia.org/wiki/Main_Page".to_string(),
    ];

    let start = Instant::now();
    let results = async_std::task::block_on(many_requests_v2(requests));
    for result in results {
        match result {
            Ok(response) => println!("*** {}\n", response),
            Err(err) => eprintln!("error: {}\n", err),
        }
    }

    println!("✅ Time Taken: {:?}", start.elapsed());
}
