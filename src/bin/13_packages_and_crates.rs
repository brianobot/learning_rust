fn main() {
    // crates
    // this is the smallest unit of code that the rust compiler considers during compilation
    // the source files we have used up until this points are all crates
    // crates can contain modules and the module might be in another file
    // that get compile with the crate

    // there are 2 forms of crates
    // binary crates and library crates

    // binary crates are programs you can compile to executables you can run
    // each must have the main function, all crates we have created so far are binary crates

    // library crates do not have a main function and are can not be compiled to executables
    // they are intended to be shared by multiple projects, most times when rustaceans says crates
    // the means library crates and they use the terms interchangable with the general programming
    // concept of a library

    // a package contains one or more crates that provide a set of functionality
    // a pakcage contains a Cargo.toml file that describes how to build the crates

    // a package can contain as many binary crates as you wish but at most only one library crates
    // a package can have multiple binary creates by placing them in the src/bin directory of the package

    
}