use std::fs;
use std::time::Duration;
use std::thread;

fn main() {
    // create directory 
    let create_dir_result = fs::create_dir("./data");

    match create_dir_result {
        Ok(_) => println!("Created Directory Successfully"),
        Err(msg) => println!("Error Creating Directory: error = {}", msg),
    };

    // sleep for 2 seconds and then delete the directory
    thread::sleep(Duration::from_secs(2));

    // remove directory
    let remove_dir_result = fs::remove_dir("./data");

    match remove_dir_result {
        Ok(_) => println!("Removed Directory successfully"),
        Err(msg) => println!("Failed to remove directory: error = {}", msg),
    }

    // we can use the path object to interact with path methods
    let my_path = std::path::Path::new("./src");
    println!("Path = {:?}", my_path);

    // some methods available on the path objects are 
    let (is_file, is_dir, is_abs, is_rel) = (
        my_path.is_file(), 
        my_path.is_dir(), 
        my_path.is_absolute(), 
        my_path.is_relative()
    );

    println!("
        Is File: {},
        Is Dir:  {},
        Is Abs:  {},
        Is Rel:  {},
    ", is_file, is_dir, is_abs, is_rel);

    println!("Path Has Root: {:?}", my_path.has_root());
    println!("Path parent: {:?}", my_path.parent());

    let dirs = fs::read_dir(".").unwrap();
    println!("Dirs: {:?}", dirs.map(|x| x.unwrap()).collect::<Vec<fs::DirEntry>>());

    let canonical = my_path.canonicalize();
    println!("Canonical: {:?}", canonical.unwrap());

    let cargo_toml_metadata = fs::metadata("./Cargo.toml").unwrap();
    println!("Metadata: {:#?}", cargo_toml_metadata);

    let file_data_as_bytes = fs::read("./Cargo.toml").unwrap();
    println!("File data: {:#?}", file_data_as_bytes);

    let file_data_as_string = fs::read_to_string("./Cargo.toml").unwrap();
    println!("File String: {}", file_data_as_string);

    let lines = file_data_as_string.lines().collect::<Vec<&str>>();
    println!("Lines = {lines:?}");
}