fn main() {
    println!("Hello, World! The Second Hello World in the learning repo");
    // why do we need to do ./bin_name to execute a binary in unix-like os

    /*
    
    this is because malicious script with the same name as global system commands
    can be placed in the directories of a user, if the current directory was added
    to the search directory for commands/shell script by defaults, users could 
    accidentally execute these commands when in those directories when they intended to
    use the system scripts with the same name, so to execute a command from
    the current directory the user must explicity specify the current directory
    as the path to check for the script
     */
}