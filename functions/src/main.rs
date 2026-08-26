/*
    - A function is a sequence of steps to be executed in order
    - A parameter is a name for an expected input to a function
    - An argument is the concrete value passed in for a parameter when the function is invoked
    - A return value is the output of a function
    - A unit is an empty tuple, a tuple without values
*/
#![allow(dead_code)]
#![allow(unused_variables)]

// Intro to functions
fn hello_world() {
    println!("Hellow World!");
}

// Functions with parameters
fn open_store(neighborhood: &str) {
    println!("Opening my pizza store in {}", neighborhood);
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {} {} pizzas!", number, topping);
}

// Explicit return values
fn square_explicit(number: i32) -> i32 {
    return number * number;
}

// Implicit return values
fn square_implicit(number: i32) -> i32 {
    number * number
}

// Unit return (empty tuple) -> let result: () = ();
fn mistery() {}

// Mix functions
fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn alphabets(text: &str) -> (bool, bool) {
    (text.contains("a"), text.contains("z"))
}

fn main() {
    // ============================================================
    // Invoke Functions
    // ============================================================
    hello_world();
    open_store("Brooklyn");
    bake_pizza(2, "pepperoni");
    println!("Explicit Square Return: {}", square_explicit(5));
    println!("Implicit Square Return: {}", square_implicit(5));
    println!("Mistery: {:?}", mistery());
    println!("Is Even: {}", is_even(8));
    println!("Is Even: {}", is_even(9));
    println!("Alphabets: {:?}", alphabets("antonio"));

    // ============================================================
    // Statements vs Expressions
    // ============================================================
    // Statements perform an action and don't return a value.
    // Expressions evaluate to a value.

    // Statement
    let x = 5;

    // Expression
    let y = {
        // Independent execution environment
        let x = 5;
        x + 1 // No semicolon -> returned by the block
    };

    println!("y: {}", y); // 6
}
