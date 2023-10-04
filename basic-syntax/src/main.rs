const GLOBAL_CONSTANT: u32 = 1;

fn basic_fn() {
    println!("This is a basic function");
}

fn main() {
    println!("This file contains my practices of basic syntax in rust");

    basic_fn();

    // Variables
    // Immutable variables
    // variables are immuatble by default
    let immutable = 1;
    println!("The value of the immutable is: {immutable}");
    // line below will cause compile-time error
    // immutable = 2;
    
    // another way to define an immutable is by using contants
    // contants are always immutable and you always need to declare the data type
    const MASS: u32 = 123;
    println!("The value of the constant MASS is: {MASS}");
    println!("The value of the global constant is: {GLOBAL_CONSTANT}");

    // variable shadowing is the act of creating new variables that uses the same name
    let x = 5;
    println!("The value of x is: {x}");
    // shadow variable
    let x = x + 1;
    println!("The value of the shadow variable x is: {x}");

    // scoped shadow variable
    // things can be scoped by wrapping them in curly brackets
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x after the inner scope is: {x}");
}
