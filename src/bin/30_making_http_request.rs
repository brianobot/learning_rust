// this crate would involve code about making http request in the rust
// docs: https://docs.rs/reqwest/latest/reqwest/

// Some other notable crates for making http request in rust are:
// 1. hyper: more low level that reqwest and it is the foundation of reqwest
// 2. curl: rust bindings for libcurl
// 3. isahc: a fast http client written in rust

// reqwest can be used in a synchronous and asynchronous pattern
use reqwest;
use reqwest::blocking::Client;

fn main() {
    let http_client = Client::new();
    
    let http_result = http_client.get("https://brianobot.github.io");
    // this does nto actually send the request, it only returns a request builder, 
    // to send the request you have to call the send method on the RequestBuilder

    let response = http_result.try_clone().unwrap().send();
    println!("REPOSNSE = {:#?}", response.unwrap());
    // usually the send method would be appended to the requestbuilder in one line

    // it is important to understand that the body of the response is not directly accessible in the response object
    // but can be accessed via some methods defined on the response object, bytes to get the body as bytes

    // in this case it would be a raw html body sent back from the request
    let body = http_result.send().unwrap().bytes();
    println!("Body = {:?}", body.unwrap());

    let post_response = http_client
        .post("http://localhost:3000/send_data")
        .body("body")
        .send()
        .unwrap()
        .bytes();
    
    println!("✅ Post Response = {:?}", post_response);
}
