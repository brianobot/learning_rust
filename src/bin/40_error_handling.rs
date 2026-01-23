#![allow(dead_code)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // panics represent something that should never happen
    // if you compile a rust program with the flag -C panic=abort
    // rust does not unwind the stack when a panic is met, it just aborts, this reduces the binary sizes of compile programs
    // because the stack info is not included

    // let cap_share = pirate_share(100, 0);
    // println!("Cap Share: {cap_share}");
    //
    // Result type
    // this represent possible failure
    //
    // catching result can be done with match expression
    let weather_report = get_weather(LatLng(0.1, 1.));

    match weather_report {
        Ok(info) => println!("Found Weather Report: {}", info.0),
        Err(_) => println!("Could Not Find Weather Report... Try Again"),
    }

    let weather_report_2 = get_weather(LatLng(10., 10.));
    if weather_report_2.is_ok() {
        println!("Weather Report 2 is Available!");
    } else {
        println!("Weather Report 2 is Not Available ⚠️");
    }

    let weather_report_3 = get_weather(LatLng(02., 23.));
    let some_value = weather_report_3.clone().ok(); // converts Result<T, E> -> Option<T>
    let err_value = weather_report_3.clone().err();
    let fallback = weather_report_3
        .clone()
        .unwrap_or(WeatherReport("Default Weather Report".to_string()));

    println!("Some_value = {some_value:?}");
    println!("Error Value = {err_value:?}");
    println!("Fallback = {fallback:?}");
    
    // if you want to return multiple types of errors from your functions 
    // you can use the generic error trait which all errors implement
    fn multiple_errors() -> Result<String, Box<dyn std::error::Error>> {
        Ok("Multiple".to_string())
    }
    
    // you can propagate errors in the main function or any function that does not return a Result type
    // how ever you can change the signature of main to return a result type and then you can propagate errors in it
    // 
    Ok(())
}

#[allow(dead_code)]
fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}

struct LatLng(f64, f64);

#[derive(Debug, Clone)]
struct WeatherReport(String);

fn get_weather(_location: LatLng) -> Result<WeatherReport, String> {
    Ok(WeatherReport(format!(
        "Info at Coordination({}, {})",
        _location.0, _location.1
    )))
}
