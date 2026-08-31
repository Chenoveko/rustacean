#[allow(unused_variables)]
#[allow(dead_code)]

fn main() {
    // ============================================================
    // Option Enum
    // ============================================================
    /* The Option enum models a scenario whree a type could be a valid value or nothing at all
        - Option::None -> Represents an absent value
        - Option::Some(T) -> Represents a present value
    The Option Enum implements the debug trait and copy if the value insides implements it
    */
    let some: Option<i32> = Option::Some(5);
    let some: Option<f32> = Option::Some(5.0);
    let some = Option::<f64>::Some(5.0);
    let none: Option<i32> = Option::None;
    println!("Option Some: {:?}", some);
    println!("Option None: {:?}", none);

    // Real example of Option Enum -> get method on an array
    let musical_instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];
    let bass: Option<&String> = musical_instruments.get(2);
    let invalid_instrument: Option<&String> = musical_instruments.get(3);
    println!("Bass: {:?}", bass);
    println!("Invalid Instrument: {:?}", invalid_instrument);

    // ============================================================
    // Unwrap and Expect Method
    // ============================================================
    /*
        - The unwrap method attempts to extract the associated data out of the Some variant
        - unwrap always assume that there is something to unwrap
        - use for developers to write faster and easier code
        - is not the safest approach
    */

    println!("Bass unwrapped: {}", bass.unwrap());
    // println!("Invalid Instrument: {}", invalid_instrument.unwrap()); -> panick because there is nothing to unwrap

    /*
        - The expect method is identical to unwrap but it allow us to customize the error message
        - If the is None Variant, the program will fail at runtime, displayin the custom error
    */
    println!(
        "Bass expected: {}",
        bass.expect("Unable to retrieve musical instrument")
    );
    // println!("Invalid Instrument: {}", invalid_instrument.expect("Unable to retrieve musical instrument")); -> panick because there is nothing to unwrap

    // ============================================================
    // match with Option Enum
    // ============================================================
    // Solve unwrap and expect
    match bass {
        Option::Some(instrument) => println!("Playing the instrument {} in my band", instrument),
        Option::None => println!("Singing with my voice"),
    }

    match invalid_instrument {
        Option::Some(instrument) => println!("Playing the instrument {} in my band", instrument),
        Option::None => println!("Singing with my voice"),
    }

    fn get_musical_instrument(instrument_option: Option<&String>) {
        match instrument_option {
            Option::Some(instrument) => {
                println!("Playing the instrument {} in my band", instrument)
            }
            Option::None => println!("Singing with my voice"),
        }
    }

    get_musical_instrument(bass);

    // ============================================================
    // Return Option enum from a function
    // ============================================================
    fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
        if item_is_in_system && item_is_in_stock {
            Option::Some(true)
        } else if item_is_in_system {
            Option::Some(false)
        } else {
            Option::None
        }
    }

    match is_item_in_stock(true, false) {
        Option::Some(true) => println!("Item is available"),
        Option::Some(false) => println!("Item is falsable"),
        Option::None => println!("Item is not available"),
    }

    // ============================================================
    // Top-Level Option Variants
    // ============================================================
    /*
    The Rust prelude is a collection of named constructs thar are available automatically in every program
    We can remove 'Option::' because are available at the top level of Rust
    */
    match is_item_in_stock(true, true) {
        Some(true) => println!("Item is available"),
        Some(false) => println!("Item is falsable"),
        None => println!("Item is not available"),
    }

    // ============================================================
    // unwrap_or Method
    // ============================================================
    // Similar to unwrap method, but mandates an argument which represents the fallback value
    let default = String::from("Default");
    println!("Bass unwrap_or: {}", bass.unwrap_or(&default));
    println!(
        "Invalid Instrument unwrap_or: {}",
        invalid_instrument.unwrap_or(&default)
    );

    // ============================================================
    // Result Enum
    // ============================================================
    /* The Option enum models the outcome of an evaluation that can produce either a success or an error
        - Result::Ok(T) -> Indicates a success. It stores an associated piece of data of generic type T
        - Result::Err(E) -> Indicates an error. It stores an associated piece of data of generic type E
    The Result Enum implements the debug trait and copy if the value insides implements it
    */
    let ok: Result<i8, &str> = Result::Ok(8);
    let disaster: Result<i8, &str> = Result::Err("Disaster");
    println!("ok: {:?} and disaster: {:?}", ok, disaster);

    // Real example of Result Enum -> parse method on a string
    let text: &str = "50";
    println!("text as number: {:?}", text.parse::<i8>());
    println!("text as bool: {:?}", text.parse::<bool>());

    // ============================================================
    // Return Result enum from a function
    // ============================================================
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err("Cannot divide by 0".to_string())
        } else {
            Ok(numerator / denominator)
        }
    }
    println!("divide by 0: {:?}", divide(20.0, 0.0));
    println!("divide ok: {:?}", divide(20.0, 12.0));

    match divide(20.0, 0.0) {
        Ok(calculation) => println!("calculation: {}", calculation),
        Err(message) => println!("Error message: {}", message),
    }

    // ============================================================
    // unwrap, expect, expect_or, is_ok and is_err with Return Enum
    // ============================================================
    // he .unwrap(), .expect(), and .unwrap_or() methods are designed to consume the Result. This means that in their definition, they take self by value, taking ownership of the variable.
    let result: Result<f64, String> = divide(20.0, 1.0);
    // as_ref() gives us a Result<&f64, &String>, so unwrap returns an &f64
    println!("Unwrap divide {}", result.as_ref().unwrap());
    println!(
        "Unwrap divide {}",
        result.as_ref().expect("Unable to parse calculation")
    );

    // Since as_ref() returns an &f64, we have to pass it a reference: &0.0
    println!("Unwrap or divide {}", result.as_ref().unwrap_or(&0.0));

    // is_ok and is_err borrow automatically, they don't need as_ref()
    println!("result ok? {}", result.is_ok());
    println!("result err? {}", result.is_err()); // Note: fixed the second is_ok to is_err

    // ============================================================
    // while let Construct
    // ============================================================
    let mut sauces = vec!["Mayonaise", "Ketchup", "BBQ"];

    // .pop() removes and returns the last element from the vector.
    // 'if let' executes the block only if .pop() successfully returns a value (Some).
    if let Some(sauce) = sauces.pop() {
        println!("The next sauce is {}", sauce)
    }

    if let Some(sauce) = sauces.pop() {
        println!("The next sauce is {}", sauce)
    }

    if let Some(sauce) = sauces.pop() {
        println!("The next sauce is {}", sauce)
    }

    // Refilling the vector to demonstrate a cleaner approach
    sauces = vec!["Mayonnaise", "Ketchup", "BBQ"];

    // 'while let' loops as long as .pop() keeps returning Some(value).
    // It automatically stops when the vector is empty and .pop() returns None.
    while let Some(sauce) = sauces.pop() {
        println!("The next sauce is {}", sauce);
    }
}
