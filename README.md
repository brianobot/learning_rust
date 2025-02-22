# 🦀 Rust Learning Repository

Welcome to my Rust Learning Repository! This repository contains code snippets and exercises that I used while following the official Rust manual. It's a comprehensive resource for anyone looking to learn Rust, from beginners to more advanced users.

## Introduction

Rust is a systems programming language that aims to provide memory safety without sacrificing performance. Known for its strong emphasis on safety, speed, and concurrency, Rust is a great language for both beginners and experienced programmers. This repository documents my journey of learning Rust through the official manual, and I hope it serves as a helpful guide for you as well.

### Notice 
```rust
cargo new package_name
```

is equivalent to

```rust
cargo new package_name --bin
```

Since both generate a binary project

In the Cargo.toml file we can specify multiple [[bin]] table to show the different binaries we want 
to generate from the project

## TL;DR Survival Guide 
1. Everyone value has an owner in rust
  ```rust
  let s1 = String::from("Brian Obot"); // here s1 one is the owner of the String
  ```
2. there can only be one owner at a time
  ```rust
  let s1 = String::from("Brian Obot");
  let s2 = s1; // here the value has been moved to a new owner s2 and s1 simply becomes inaccessible since 
  // it has not value now, it is snapped out of reality by Rust Thanos (s1 is invalidated )
  ```
  Moving values also applies to function, since passing values to function call is actually the same as moving the 
  value into the variable in the function call signature

  ```rust
  let s2 = String::from("Brian Obot");

  fn print_string(s: String) {
    println!("{}", s);
  }

  print_string(s2); // here the value from the string is moved inot s in the function and s2 is invalidated 
  ```
3. When the owner goes out of scope, the value would be dropped
   ```rust
    // external scope 
    // scope refers to the region of code between curly brackets
   {
        // inside the curly braces it's own little scope
        let secret = vec![1, 2, 3, 4].to_bytes();
   } // aat this point, secret is going out of scope and it would be invalidated
   ```

we can use the process of creating reference to values without taking ownership of the value
this can be done with the &<variable_name> syntax, creating reference is called ```borrowing```

### Borrowing Rules
- At any given time you can only have one mutable reference or any number of imutable references
- references must always be valid (the borrow checker uses lifetimes to ensure references are always valid)
  - this ensures that the references lifetime does not outlive the lifetime of the value

## Contributing

I welcome contributions from anyone! If you have any improvements or additional exercises you'd like to share, please feel free to fork this repository and submit a pull request.