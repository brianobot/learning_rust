use std::io;


fn main() {
    println!("Welcome to the Abbreviation Game: ");
    println!("===================================");

    let username = input("Enter your username: ");
    let is_ready = input("Are you ready: ");
    let mut score: i32 = 0;
    
    if is_ready.to_lowercase() != String::from("yes") {
        println!("Game ended\n");
        println!("++++++++++++++");
        return ()
    }

    let abbrs = ["CPU", "RAM", "ROM", "GPU", "SI"];
    let fullforms = [
        "central processing unit",
        "random access memory",
        "read only memory",
        "graphics processing unit",
        "System international",
    ];

    for (i, abbr) in abbrs.into_iter().enumerate() {
        let result = ask_question_and_get_reply(&abbr);

        if result == fullforms[i] {
            score += 1;
            println!("Yay! You are Correct!");
        } else {
            println!("Incorrect! Try Again Later");
        }
    }

    println!("Game Ended: {username} Scored [{score}/{}]", abbrs.len());

}

fn ask_question_and_get_reply(abbr: &str) -> String {
    let mut reply = String::new();

    println!("What is the Full Meaning of {}: ", abbr);

    io::stdin()
        .read_line(&mut reply)
        .expect("Failed to input");

    reply.trim().to_lowercase().clone()
}


fn input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read input");

    let value = value.trim();

    value.to_string()
}
