fn main() {
    // generics allows us to use a generic type in place of a concrete type
    // to return conde duplication based on concrete types

    // the simplest and most use of generic is for type parameters
    #[allow(dead_code)]
    fn foo<T>(arg: T)  -> T {
        arg
    }

    // in the case above, we defined the generic in the angle brackets and then proceed to use it
    // in the annotation of the function parameter type and return value

    let _result_i32 = foo(1);
    let _result_u8 = foo(12u8);
    let _result_string = foo(String::from("Brian"));
    // the same function has been reused for different datatype, greatly simplying the codebase

    /*
    Using generics, we can write code that can be used with multiple data types without 
    having to rewrite the same code for each data type, making life easier and coding less error-prone.
     */

    let number_list = vec![1, 20, 3, 4, 5, 6];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number
        }
    }

    println!("The largest number is {largest}");

    let second_list = vec![23, 4, 5, 23, 5, 23, 523, 523, 12];
    let third_list = vec![23, 86, 5, 435, 5, 23, 45, 42, 12];

    // this process above works, but say we had another list to find the largest number in
    // we could duplicate this logic and it would work as well, but then this introduces
    // an issue with duplicates since any change that must be done to the logic must be also
    // done to the duplicates, this is error prune and tiring, so one way is to asbtract the logic away into a function

    largest = largest_number(&second_list);
    println!("The largest number in second list is {largest}");
    largest = largest_number(&third_list);
    println!("The largest number in third list is {largest}");

    // now the same way functions allowed us to abstract away a logic to reduce duplication
    // generics allow use to abstract the signature of those function so that they can operate on an abstract type
    // and not a concrete type

}


fn largest_number(lst: &Vec<i32>) -> &i32 {
    // 💡 Note that the local variable largest_number is the same as the functions name
    // rust allows this because it knows that the local variable is not the function
    // since it was not used in the calling context, function_name()
    // this feature/characteristic allow for variables within functions to shadow the function
    // name without brekaing the code, when called from outside the function, 
    // the local variable is not accessible so the function is called instead

    let mut largest_number = &lst[0];

    for number in lst {
        if number > largest_number {
            largest_number = number
        }
    }

    largest_number
}