#![allow(unused_variables)]

fn main() {
    // ============================================================
    // Scope and Ownership
    // ============================================================
    // When the variable goes out of scope, the owner deallocates the memory of that value
    {
        let age: i8 = 32; // "age" variable is the owner of the value
    } // "age" variable goes out of scope

    // ============================================================
    // Copy Trait
    // ============================================================
    // The copy trait mandates that a type can be copied, which means that a full duplicate can be created
    // Rust's primitive data types, or the fixed size ones that we store on the stack like integers,floats... implement this trait
    let time = 2025;
    let year = time; // Full copy of time -> 2 duplicate, separate, independent copies of the value
    // 2 owners, "time" is responsible for cleaning up it's stack entry and "year" for his entry
    println!("The time is {}. The year is {}", time, year);
    println!("The time address is {:p}. The year address is {:p}", &time, &year); // {:p} -> formats a reference/pointer as a memory address.

    // ============================================================
    // Move of Ownership
    // ============================================================
    // A move is the transfer of ownership from one owner to another
    // One owner at a time, but the owner can change
    let person: String = String::from("Boris");
    let genius: String = person; // "genius" take ownership of person -> the stack entry is copied from "person" to "genius"; "person" is no longer valid.

    // ============================================================
    // Drop Function
    // ============================================================
    // drop() deallocates the memory on the heap
    drop(genius);
    // println!("{}", genius);  ❌ Error

    // ============================================================
    // Clone Method
    // ============================================================
    // clone() creates a deep copy of the value, including heap data.
    let person: String = String::from("Boris");
    let genius: String = person.clone();
    println!("Person: {}, Genius: {}", person, genius);

    // ============================================================
    // References and Borrowing
    // ============================================================
    // A reference allows the program to use a value without moving ownership
    // Borrowing means using something without taking ownership of it ("&" borrow operator)
    let car: String = String::from("Toyota");
    let car_reference: &String = &car;
    println!("My car: {}", car_reference);
    /*
    More technically, a reference is a type of pointer.
    In Rust, a reference is guaranteed to point to a valid value
    for the lifetime of that reference.

    In comparison, a raw pointer does not have that guarantee.
    */

    // ============================================================
    // Dereference Operator (*)
    // ============================================================
    // An operator is a symbol that applies an operation to a value
    // To dereference means to access the data at the memory address that the reference point to
    // The dereference operator (*) operates on a reference.
    let my_value = 2;
    let my_reference = &my_value;
    println!("{}", my_value); // i32  -> Display
    println!("{}", *my_reference); // &i32 -> i32 -> Display
    println!("{}", my_reference); // &i32 -> Display works through the reference
    // Explicit dereferencing is not necessary here because Display Trait is implemented for references when the referenced value implements Display.

    // ============================================================
    // String Types
    // ============================================================
    /*
        - String literal -> hardcoded, read-only piece of text encodes in the binary
            * Embedded directly into the binary executable (know at compile time)
            * Has type &str -> reference to that hard coded text
        - String Slice (&str) -> reference to the text in the memory that has loaded the binary file
            * fixed content
        - String -> dynamic piece of text stored on the heap at runtime
            * owned string
            * can grow/change
            * owns heap data
            * Value lives on the heap
            * Stack entry (3 pieces of data) -> pointer/reference, length  and capacity
        - &String -> reference to a heap String
    */
    let food: &str = "pasta"; // String Literal 
    let text: String = String::new(); // Creates an empty String
    let candy: String = String::from("KitKat"); // Creates a String from a string literal

    // push_str() method appends a string slice (&str) to a String.
    let mut name: String = String::from("Graydon");
    println!("Name: {}", name); // println! borrows the value; it does not take ownership
    name.push_str(" Hoare");
    println!("Name and surname: {}", name);

    // ============================================================
    // Copy Trait with References
    // ============================================================
    // Remember that stack types implement the Copy Trait
    // Refrences in Rust implement the Copy Trait as well
    let ice_cream: &str = "Cookies and Cream";
    let ice_cream_copy = ice_cream; // Copying a reference creates another reference to the same data.
    println!("My ice cream: {:p}. My ice cream copy: {:p}", ice_cream, ice_cream_copy);

    // ============================================================
    // Ownership and Function Parameters
    // ============================================================
    fn print_my_value_int(value: i32) {
        println!("Your values is {}", value);
    }
    fn print_my_value_str(value: String) {
        println!("Your values is {}", value);
    }
    // Without transfer of ownership
    let apples = 24;
    print_my_value_int(apples); // The function recevie a copy of apples, no transfer of ownership
    println!("Apples is still valid {}", apples);

    // With transfer of ownership
    let oranges: String = String::from("Oranges");
    print_my_value_str(oranges); // Ownership moves from oranges to the function parameter
    // println!("{}", oranges); ❌ Error: oranges has been moved

    // ============================================================
    // Mutable Parameters
    // ============================================================
    fn add_fries(mut meal: String) {
        meal.push_str(" and Fries");
        println!("My meal: {}", meal);
    }

    let burguer: String = String::from("Burguer");
    add_fries(burguer);
    // println!("{}", burger); // ❌ Error: burger was moved

    // ============================================================
    // Return Values
    // ============================================================
    // A function can transfer ownership by returning a value
    fn bake_cake() -> String {
        let cake: String = String::from("Chocolate Mousse");
        cake // Ownership moves from the function to the caller
    }

    let cake = bake_cake();
    println!("I now have a {} cake", cake);
}
