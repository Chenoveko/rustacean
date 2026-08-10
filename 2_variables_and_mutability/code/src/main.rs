type Meters = i32; // Type alias

// Constants -> name assigned to a value. A constant's value cannot change. Value must be know at compile time. Require explicit type declaration
    /*
    Constants vs immutable variables
        - Variables are limited to a function scope
        - Constants can be declared at any scope
    */
const _TAX_RATE: f32 = 0.5;

fn main() {
    // Variables
    let apples = 50;
    let oranges: i32 = 25;
    let _fruits = apples + oranges; // _ for unused variables

    // Interpolation with curly braces
    println!("My garden has {} apples.", apples);
    println!("My garden has {apples} apples.");
    println!("This year my gardes has {} apples and {} oranges", apples, oranges);
    println!("This year my gardes has {0} apples and {1} oranges. I can't believe I have {0} apples.", apples, oranges); // Positional arguments

    // Mutable and immutable variables -> Variables are immutable by default
    let mut gym_reps = 10; // Value can change, type can't change
    println!("I plan to do {} reps.", gym_reps);
    gym_reps = 12;
    println!("Now I plan to do {} reps.", gym_reps);

    // Variable shadowing -> Means redeclaring a variable. The original variable is "replaced" by the new one
    let grams_of_protein: &str = "100.345";
    println!("Grams of protein in string type {}", grams_of_protein);
    let grams_of_protein: f64 = 100.345;
    println!("Grams of protein in float type {}", grams_of_protein);
    let mut grams_of_protein: i32 = 100;
    println!("Grams of protein in integer type {}", grams_of_protein);
    grams_of_protein = 105;
    println!("New grams of protein in integer type {}", grams_of_protein);

    // Scope -> boundary or region of code where a name is valid 
    let macchiato_price = 4.99;
    {
        // Block -> area between an opening curly brace and a closing curly brace 
        let capuccino_price = 5.99;
        println!("Capuccino price -> {} $", capuccino_price);
        println!("Macchiato price -> {} $", macchiato_price);
    }
    // println!("Capuccino cost -> {} $", coffe_price); capuccino_price out of scope

    // Type aliases -> alternate name that we can assign to an existing type
    let mile_race_length: Meters = 1600; // type alias on top
    println!("The race is {} miles long.", mile_race_length);

    // Compiler directives -> annotation that tells the compiler how to parse the source code
    /*
        - We can apply directives to individual lines, to functions or even to entire Rust files
        - We write the directive on the line above the entity that we want to apply the directive
        - If we want to apply the directive to the whole file, we have to put the directive at the top and add ! -> #![allow(unused_variables)]
    */
    #[allow(unused_variables)]
    let unused_variable = 15;
}
