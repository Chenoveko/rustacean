#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    // ============================================================
    // String Slice from a String
    // ============================================================
    let action_hero: String = String::from("Arnold Schwarzenegger");
    let action_hero_reference: &String = &action_hero; // String Reference to heap allocated string
    let first_name: &str = &action_hero[0..6]; // String Slice, with syntactic shortcut &action_hero[..6];
    let last_name: &str = &action_hero[7..]; // String Slice 
    let full_name: &str = &action_hero[..]; // String Slice 
    println!("Action hero: {}", action_hero_reference);
    println!("Action hero first name: {}", first_name);
    println!("Action hero last name: {}", last_name);
    println!("Action hero full name: {}", full_name);

    // ============================================================
    // String Slices and String Literals
    // ============================================================
    let action_hero: &str = "Arnold Schwarzenegger"; // String literal containing a reference to a piece of text stored in the binary executable
    let first_name: &str = &action_hero[0..6]; // String Slice 
    let last_name: &str = &action_hero[7..]; // String Slice 
    println!("Action hero: {}", action_hero);
    println!("Action hero first name: {}", first_name);
    println!("Action hero last name: {}", last_name);
    /*
    This does not create a dangling reference because the string literal is stored in the binary executable and lives for the entire program

    let first_name: &str = {
        let action_hero: &str = "Arnold Schwarzenegger";

        &action_hero[0..6]
    };

    println!("Action hero first name: {}", first_name);
    */

    // ============================================================
    // String Slice Lengths
    // ============================================================
    // The length of a string slice refers to a count of its bytes, not it's characters
    // 1 character occupies 1 byte but emoji occupies 4 bytes
    let food: &str = "pizza"; 
    let food_slice: &str = &food[0..3]; 
    let food_emoji: &str = "🍕"; 
    // let food_emoji_slice: &str = &food_emoji[0..2]; Panicked, not a valid UTF-8 character boundary
    println!("Food Length: {}", food.len());
    println!("Food Slice Length: {}", food_slice.len());
    println!("Food Emoji Length: {}", food_emoji.len());

    // ============================================================
    // String Slices as Function Parameters
    // ============================================================
    // The most versatile way to use strings in functions is with string slices
    fn do_hero_stuff(hero_name: &str) {
        println!("{} saves the day 🦸‍♂️", hero_name);
    }
    let action_hero: String = String::from("Arnold Schwarzenegger");
    do_hero_stuff(&action_hero); // Rust supports this through deref coercion: &String is automatically converted to &str. However, Rust does not automatically convert &str to &String
    let another_action_hero: &str = "Sylvester Stallone";
    do_hero_stuff(another_action_hero);

    // ============================================================
    // Array Slices
    // ============================================================
    let my_array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let array_slice: &[i32] = &my_array[0..2]; // Reference to some chunk or portion of an array
    let full_array_slice: &[i32] = &my_array[..]; // Reference to full array
    let array_ref: &[i32; 6] = &my_array; // Reference to a 6 element of array of i32 (more restricted)

    // ============================================================
    // Deref Coercion with Array Slices
    // ============================================================
    fn print_array_length(reference: &[i32]) {
        println!("array len: {} ", reference.len());
    }
    print_array_length(&my_array); // Rust converts &[i32; N] to &[i32] through deref coercion.
    print_array_length(array_slice);
    print_array_length(full_array_slice);
    print_array_length(array_ref); // Rust converts &[i32; N] to &[i32] through deref coercion.

    // ============================================================
    // Mutable Array Slices
    // ============================================================
    // Rust does not permit mutable slices of string. However Rust does permit mutable slices of arrays
    let mut mutable_array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let mut_array_slice: &mut [i32] = &mut mutable_array[2..4];

    mut_array_slice[0] = 35;
    println!("My array modified using mut slices: {:?}", mutable_array);

    
}
