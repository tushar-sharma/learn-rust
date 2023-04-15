fn main() {
    // Macros in Rust are a powerful feature 
    // that allows you to write code that generates other code at compile-time. 
    // In the case of the println! macro, 
    // it takes a string literal with placeholders for values to be printed 
    // and generates code to print the formatted output to stdout at compile-time.

    //example of a macro 
    let distance = 2;
    println!("The distance is {}", distance);

    //the println function is a regular function that takes a single string argument
    // and prints it to stdout without any formatting
    println("The distance is ", distance);

    //println! macro is used for printing formatted output, 
    //while the println function is used for printing a single string without any formatting.
    println("The distance is 2");

}