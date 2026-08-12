use std::ops::Range;
use std::ops::RangeInclusive;

/*
    - Rust is statically typed
    - The compiler can infer the types of variables based on their initial assignments
    - Scalar type holds a single value -> integers, floats, bools and characters
*/
#[allow(unused_variables)]
fn main() {
    // ============================================================
    // Integers (Signed and Unsigned)
    // ============================================================
    // Signed integers -> i8, i16, i32, i64, i128 and isize (depends on the system architecture: 32 or 64 bits)
    // Unsigned integers -> u8, u16, u32, u64, u128 and usize (depends on the system architecture: 32 or 64 bits)
    let eight_bit_signed: i8 = -128; // i32 require 32 bits (4 bytes) of mem
    // let eight_bit_signed: i8 = 225; -> literal out of range for i8 type i8 whose range is `-128..=127`
    let some_value = 20i8; // Another type explicit declaration
    let sixteen_bit_signed: i16 = 32_500; // Using _ as visual separator for numbers
    let days: usize = 55;
    let years: isize = -15_000;

    // ============================================================
    // String Literals
    // ============================================================
    // String literal -> A value written directly in the source code and known at compile time.
    println!("Hello world"); // String literal 
    println!("Dear Emily,\nHow have you been?"); // \n -> New line.
    println!("\tOnce upon a time"); // \t -> Tab.
    println!("Juliet said \"I love you Romeo\""); // \" -> Double quote.
    let filepath = "C:\\My Documents\\new\\videos"; // \\ -> Backslash.
    println!("{filepath}");
    // Raw string -> Escape sequences are not processed.
    // Backslashes can be written directly without escaping them.
    let raw_filepath = r"C:\My Documents\new\videos";
    println!("{raw_filepath}");

    // ============================================================
    // Methods
    // ============================================================
    // A method is a function that lives on a value. It's an action we can ask the value to execute
    let value: i32 = -15;
    println!("ABS Value {}", value.abs());
    println!("Pow 2 Value {}", value.pow(2));
    let empty_space = "                         my content                 ";
    println!("{}", empty_space.trim());

    // ============================================================
    // Floats
    // ============================================================
    // f32 -> 6-9 digits of precision
    // f64 -> 15-17 digits of precision
    let pi: f64 = 3.1415932312313131;
    println!("The current value of pi is {}", pi);
    println!("The current value of pi floored is {}", pi.floor()); // Rounds down -> 3
    println!("The current value of pi ceiling is {}", pi.ceil()); // Rounds up -> 4
    println!("The current value of pi rounded is {}", pi.round()); // Rounds to the nearest integer -> 3
    // Format specifier customizes the printed representation of the  interpolated value
    println!("The current value of pi formatted is {:.3}", pi);

    // ============================================================
    // Casting
    // ============================================================
    /*
    Casting -> Converting a value from one type to another using the as keyword.
        - Value must fit within the constraints of the new assigned type
        - Be careful: casting to a smaller or incompatible numeric type may lose information.
    */
    let miles_away: i32 = 50;
    let miles_away_i8 = miles_away as i8;
    let miles_away_u8 = miles_away as u8;
    let miles_away_f32 = miles_away as f32;
    println!("Miles Away as i32 {}", miles_away);
    println!("Miles Away as i8 {}", miles_away_i8);
    println!("Miles Away as u8 {}", miles_away_u8);
    println!("Miles Away as f32 {:.2}", miles_away_f32);

    // ============================================================
    // Math Operations
    // ============================================================
    let addition = 5 + 4;
    let subtraction = 10 - 6;
    let multiplication = 3 * 4;
    let floor_division = 5 / 3; // Floor division -> integer divide by integer
    let float_division = 5.0 / 3.0; // Decimal division
    let modulo = 7 % 2; // remainder operator
    println!(
        "Addition: {}, Subtraction: {}, Multiplication: {}, Floor Division: {}, Float Division: {:.2}, Modulo: {}",
        addition, subtraction, multiplication, floor_division, float_division, modulo
    );

    // ============================================================
    // Augmented Assignment Operators
    // ============================================================
    // Augmented assignment operators -> Perform an operation and assign the result back to the same variable.
    let mut year = 2026;
    year += 1; // Equivalent to: year = year + 1;
    println!("Next year: {}", year); // 2027
    year -= 1; // Equivalent to: year = year - 1;
    println!("Current year: {}", year); // 2026
    year *= 2;
    println!("Future year: {}", year); // 4052
    year /= 2;
    println!("Actual year: {}", year); // 2026
    // Also x %= 5;

    // ============================================================
    // Booleans
    // ============================================================
    let is_handsome = true;
    let is_silly = false;
    println!("Hndsome: {}, Silly: {}", is_handsome, is_silly);
    let age: i32 = 21;
    let is_young = age < 35;
    println!("Young: {}", is_young);
    println!("Age positive: {}, Age negative: {}", age.is_positive(), age.is_negative());
    let is_true = true;
    println!("Bool inversion: {}", !is_true);
    println!("Equality operator: {}", "Coke" == "Pepsi");
    println!("Inequality operator: {}", "Coke" != "Pepsi");
    println!("And operator: {}", true && false);
    println!("Or operator: {}", true || false);

    // ============================================================
    // Character
    // ============================================================
    /*
        - Represents a single unicode character
        - unicode is a computing standard for the representation of text for most of the world's writing system
        - Use single quotes
    */
    let first_initial: char = 'C';
    let crab: char = '🦀';
    println!("Character methods");
    println!("{}, {}", first_initial.is_alphabetic(), crab.is_alphabetic());
    println!("{}, {}", first_initial.is_uppercase(), crab.is_uppercase());
    println!("{}, {}", first_initial.is_lowercase(), crab.is_lowercase());

    // ============================================================
    // Arrays
    // ============================================================
    // Fixed-size collection of homogeneous data (data of the same type)
    let numbers: [i32; 4] = [1, 2, 3, 4];
    let apples: [&str; 3] = ["Granny Smith", "McIntosh", "Red Delicious"];
    println!("Length of numbers: {}, Length of apples: {}", numbers.len(), apples.len());
    let currency_rates: [f32; 0] = []; // Empty array
    let mut seasons: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];
    let [spring, summer, fall, winter] = seasons; // Array destructuring
    let [first, _, _, last] = numbers; // Array destructuring ignoring elements
    println!("First: {}, Last: {}", first, last);
    // Reading and writing array elements
    println!("American Seasons: {} {} {} {}", seasons[0], seasons[1], seasons[2], seasons[3]);
    seasons[2] = "Autumn";
    println!("British Seasons: {} {} {} {}", seasons[0], seasons[1], seasons[2], seasons[3]);

    // ============================================================
    // Traits
    // ============================================================
    /*
    Basics of Traits:
        - A trait is a contract that requires that a type support one or more methods
        - Traits establish consistency between types; methods that represent the same behavior have the same name
        - When a type opts in to honoring a trait's requirements, we say the type implements the trait
        - Types can vary the implementation but still implement the same trait
        - A type can choose to opting in to implementing a trait
        - A type can implement multiple traits. There are hundreds of traits available in Rust
        - A trait is called an interface or protocol in other programming languages
    */
    // ============================================================
    // Display Trait
    // ============================================================
    /*
        - The Display trait requires that a type can be represented as a user-friendly, readable string
        - The Display trait mandates a format method that returns the string
        - When we use the {} interpolation syntax, Rust relies on the format method
        - Integers, floats and booleans will implement the Display trait so we are able to interpolate them with curly braces
        - It is not always clear how a complex type should be represented as a piece of text
        - Not all types implement the Display trait. One example is the array type
    */
    println!("{}", 42);

    // ============================================================
    // Debug Trait
    // ============================================================
    /*
        - The Debug trait is used for developer-oriented representations of values.
        - {:?} -> Debug formatting.
        - {:#?} -> Pretty-printed Debug formatting.
        - Arrays implement Debug, so they can be printed using {:?}.
    */
    // : introduces a format specifier.
    println!("Debug Trait -> {:?}", seasons);
    println!("Pretty-print -> {:#?}", seasons);

    // ============================================================
    // Debug Macro
    // ============================================================
    /*
        - Prints and returns the value of a given expression for quick and dirty debugging (for development)
        - It uses the Debug Traits's format method to output several helpfull deatils about the content we pass in here
        - The argument that we pass to the dbg! macro must implement the Debug Trait so that Rust can print out
    */
    // Prints and returns the value of a given expression for quick and dirty debugging
    // It uses the Debug Traits's format method to output several helpfull deatils about the content we pass in here
    println!("Debug Macro");
    dbg!(2 + 2);
    dbg!(seasons);

    // ============================================================
    // Tuples
    // ============================================================
    // Fixed-size collection that can contain values of different types
    let employee: (&str, i32, &str) = ("Molly", 32, "Marketing");
    // Access tuple elements by index
    // let name = employee.0;
    // let age = employee.1;
    // let department = employee.2;
    let (name, age, department) = employee; // Tuple destructuring
    println!("Name: {}, Age: {}, Department: {}", name, age, department);
    // Tuples don't implement the Display trait, but they do implement the Debug Trait
    println!("{:?}", employee);
    // println!("{}", employee); This doesn't work because tuples don't implement Display Trait

    // ============================================================
    // Ranges
    // ============================================================
    // A range is a sequence/interval of consecutive values
    let week_days: Range<i32> = 1..7; // Upper value excluded
    let week_days_inclusive: RangeInclusive<i32> = 1..=7; // Upper value included
    let letters: Range<char> = 'b'..'f';
    // Ranges don't implement the Display trait, but they do implement the Debug trait
    println!("Week Days: {:?}", week_days);
    println!("Week Days Inclusive: {:?}", week_days_inclusive);
    // println!("{}", week_days); This doesn't work because ranges don't implement Display Trait
    // Iterate over a range
    for number in week_days {
        println!("{number}")
    }
    for letter in letters {
        println!("{letter}")
    }

    // ============================================================
    // Generics
    // ============================================================
    /*
        A generic is a type parameter that allows code to work
        with different types while preserving type safety.

        Range<T> is generic:
            - Range<i32>  -> T is i32
            - Range<char> -> T is char
    */
}
