fn main() {
    // variable assignment is done using 'let' keyword
    // no type annotation is required as Rust has local type inference
    let age = 10i8;

    // we can skip annotating the literal if you give an explicit type 
    let shoe_size : i8 = 12;

    println!("I wear a size {} shoe", shoe_size);

    // variable are mutable by default
    // to reassign a value you must declare it with 'mut' keyword

    let mut height = 24i8; 
    height = 15i8; 
    println!("My height is {}", height);
}