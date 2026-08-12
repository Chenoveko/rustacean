// ============================================================
// Constants
// ============================================================
/*
A constant is a name assigned to a value that cannot change.
Its value must be known at compile time.
Constants require an explicit type declaration.

Constants vs. immutable variables:
- Variables declared with let are scoped to the block where they are declared.
- Constants can be declared in any scope, including the global scope.
- Constants use const and require an explicit type.
*/
const _TAX_RATE: f32 = 0.5;

fn main() {
    // ============================================================
    // Variables
    // ============================================================
    let apples = 50;
    let oranges: i32 = 25;
    let _fruits = apples + oranges; // _ for unused variables

    // ============================================================
    // Interpolation with Curly Braces
    // ============================================================
    // 3 forms -> Sequential placeholders, Variable capture and Positional arguments
    println!("My garden has {} apples.", apples); // Placeholder
    println!("My garden has {apples} apples."); // Captured variable
    println!(
        "This year my gardes has {} apples and {} oranges",
        apples, oranges
    ); // Multiple placeholders
    println!(
        "This year my gardes has {0} apples and {1} oranges. I can't believe I have {0} apples.",
        apples, oranges
    ); // Positional arguments

    // ============================================================
    // Mutable and Immutable Variables
    // ============================================================
    // Variables are immutable by default!
    let mut gym_reps = 10; // Value can change, type can't change
    println!("I plan to do {} reps.", gym_reps);
    gym_reps = 12;
    println!("Now I plan to do {} reps.", gym_reps);

    // ============================================================
    // Variable Shadowing
    // ============================================================
    // Variable shadowing -> Declaring a new variable with the same name.
    // The new variable shadows the previous one and can have a different type.
    let grams_of_protein: &str = "100.345";
    println!("Grams of protein in string type {}", grams_of_protein);
    let grams_of_protein: f64 = 100.345;
    println!("Grams of protein in float type {}", grams_of_protein);
    let mut grams_of_protein: i32 = 100;
    println!("Grams of protein in integer type {}", grams_of_protein);
    grams_of_protein = 105;
    println!("New grams of protein in integer type {}", grams_of_protein);

    // ============================================================
    // Scope
    // ============================================================
    // Scope -> boundary or region of code where a name is valid
    let macchiato_price = 4.99;
    {
        // Block -> area between an opening curly brace and a closing curly brace
        let capuccino_price = 5.99;
        println!("Capuccino price -> {} $", capuccino_price);
        println!("Macchiato price -> {} $", macchiato_price);
    }
    // cappuccino_price is out of scope here.
    // println!("Capuccino cost -> {} $", coffe_price); capuccino_price out of scope

    // ============================================================
    // Type Aliases
    // ============================================================
    // Type aliases -> alternate name that we can assign to an existing type
    // Type aliases can also be declared at the top of the file, outside main, so they can be used by other functions within the module.
    type Meters = u32;
    let mile_race_length: Meters = 1600; // type alias on top
    println!("The race is {} miles long.", mile_race_length);

    // ============================================================
    // Compiler Attributes
    // ============================================================
    /*
    Attribute -> Metadata that provides instructions or information to the compiler
        - They can control compiler behavior, configure lints, mark tests, enable conditional compilation, etc
        - Attributes use the syntax #[...] (Outer attributes) or #![...] (Inner attributes)
        - `#[...]` -> Outer attribute: Applies to the item that comes immediately after it.
        Example:
        #[allow(unused_variables)]
        let x = 10; // The attribute applies only to `x`
        - `#![...]` -> Inner attribute: Applies to the container where the attribute is written.
        Example:
        #![allow(unused_variables)]
        If placed at the top of the file, it applies to the entire crate.
        - Summary
            * `#[...]` -> Applies to what comes NEXT.
            * `#![...]` -> Applies to what CONTAINS it.
    */
    #[allow(unused_variables)]
    let unused_variable = 15;
}
