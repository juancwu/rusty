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

    // Data Types
    //
    // Integer Types (starts with i is signed, starts with u is unsigned)
    // size | signed | unsigned |
    // 8-bit i8 u8
    // 16-bit i16 u16
    // 32-bit i32 u32
    // 64-bit i64 u64
    // 128-bit i128 u128
    // arch isize usize

    // building in debug overflowing variables will go into "panic" - exit with error
    // building in release, overflow will wrap around
    
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    
    println!("Default floating point is 64-bit long: {x}");
    println!("This is a 32-bit floating-point: {y}");

    // Boolean
    let falsy = false;
    let truthy = true;

    println!("Normal boolean values, falsy: {falsy}, truthy: {truthy}");

    // Character type
    let c = 'z';
    let z: char = 'a';
    let emoji = '👍';

    println!("Character type: {c}, {z}, {emoji}");

    // Tuple
    let tup = (100, 2.1, 1);
    let (x, y, z) = tup; // deconstruction
    println!("The value of x, y, and z are: {x}, {y}, {z}");


    // Arrays
    let months = [
        "Jan",
        "Feb",
        "Mar",
        "May",
        "Apr",
        "Jun",
        "Jul",
        "Aug",
        "Sep",
        "Oct",
        "Nov",
        "Dev"
    ];
    
    println!("Months in an array: {:?}", months);

    // let typed_array: [i32; 4] = [1, 2, 3, 4];
    // let initial_value_array: [3; 5];
}
