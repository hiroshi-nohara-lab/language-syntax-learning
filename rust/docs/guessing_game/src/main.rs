// to obtain user input and then print the result as output, we need to bring the io input/output library into scope. The io library comes from the standard library, known as std:
// by default, rust has a set of tiems defined int he standard library that it brings into the scope of every program. This set is called prelude
use std::io;
// The Rng here is at rait, that defines methods that random number generator implements, and 
use rand::Rng;



fn main() {
    println!("Guess the number!");

    let secret_number=rand::rng()
                        //   this is a method defined by the Rng trait
                        // the argument is start..=end format, inclusive on the lower and upper bounds
                          .random_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    // storing values with variables
    // in rust variables are immutable by default means, once the value is assigned the value won't change.
    // let apples=5;
    // to make the variables mutable we use the mut keyword
    // in the following code the guess game will binded to the result of calling String::new, a function that returs a new instance of String.
    //String is a string type provided by the standard library
    // the :: syntax in the ::new  line indicates that new is an associated function of the String type.
    // An associated function is a function that's implemented on a type, in this case String. The new function creates a new, empty string.

    // so this line does is creates a mutable variable that is currently bound to a new, empty instance of a String.
    let mut guess = String::new();

    println!("You guessed:{}", guess);

    // & here tells the argument here is a reference, will be pointing to the address of the  original variable
    // like variables, references are  immutable by default. Hence you need to write &mut guess rather than &guess to make it mutable
    io::stdin()
        // read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value. Result is an enumeration, often called an enum, which is a type that can be of multiple possible states, we call each possible state as variant.
        //  The purpose of REsult types is to encode error-handling information.
        //  The variants are Ok and Err
        .read_line(&mut guess)
        // so calling read_line will return Result value, which have an expect method that you can call 
        // how expect works is that if the Result variant is Ok, expect will take the return value and return it just that value to you for future use. 
        // in this case that value is the number of bytes in the user's input
        // if you don't call expect the program will compile but with warning 
        .expect("Failed to read line"); 


    println!("You guessed:{} and multiple by 3 will be {}", guess,guess);
    // so printing a variable can be done in two ways
    println!("This was your guess : {guess}");
    //  let x=5;
    //  let y=10;
    //  println!("x is {x} y + 2 {}", y+2); because we want to print an expression or an evaluated value we use y+2 in the second argument
}



