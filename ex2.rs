// the following imports a std::fmt module
// this will generate a warning as it's unused
use std::fmt;

fn main() {
    //i8 suffix indicates that this an 8-bit signed integer
    // Variables are immutable by default
    let distance = 100i8;

    println!("You are {} miles away", distance);

    //unlike c we do not return some int from the main function
    //Main's return type is '()' read as unit
}
