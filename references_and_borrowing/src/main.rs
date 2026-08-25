#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    // ============================================================
    // Immutable and Mutable Reference Parameters
    // ============================================================
    // Function with an immutable reference; it can read the value but cannot modify it, and it does not take ownership.
    fn show_my_meal(meal: &String) {
        println!("Show my meal: {}", meal);
    }

    // Function with a mutable reference; it can read and modify the value, but it does not take ownership.
    fn add_flour(meal: &mut String) {
        meal.push_str(" Add flour");
    }

    let mut current_meal: String = String::new();
    add_flour(&mut current_meal);
    show_my_meal(&current_meal);

    // ============================================================
    // Multiple Immutable References
    // ============================================================
    let car: String = String::from("Ferrari");
    // Rust permits any number of immutable references to the same value
    let car_ref1: &String  = &car;
    let car_ref2: &String = &car;
    println!("My references to the car: {} and {}", car_ref1, car_ref2);

    // ============================================================
    // Mutable References Restrictions
    // ============================================================
    // Lifetime: how long the referenced value or borrow remains valid.
    let mut motorbike: String = String::from("Kawasaki");
    // Rust only permits a single mutable references to the same value at a time
    let motorbike_ref1: &mut String  = &mut motorbike;
    motorbike_ref1.push_str(" Ninja");

    // let motorbike_ref2: &String = &motorbike; An immutable and a mutable reference cannot exist at the same time.
    // println!("My motorbike references: {}", motorbike_ref1, motorbike_ref2);

    // A new mutable reference can be created after the previous reference is no longer active.
    let motorbike_ref3: &mut String = &mut motorbike;
    motorbike_ref3.push_str(" ZX10R");
    
    println!("My motorbike: {}", motorbike);

    // ============================================================
    // Ownership with Immutable and Mutable References
    // ============================================================
    let coffee: String = String::from("Mocha");
    // Immutable references implement the copy trait 
    let coffee_ref1: &String = &coffee;
    let coffee_ref2 = coffee_ref1; // Full copy of the reference
    println!("Coffee refs: {} and {}", coffee_ref1, coffee_ref2);

    let mut soda: String = String::from("Cola");
    // Mutable references do not implement the copy trait 
    let soda_ref1: &mut String = &mut soda;
    let soda_ref2 = soda_ref1; // Ownership of the mutable reference is moved
    soda_ref2.push_str(" Zero");

    println!("Soda: {}", soda);

    // ============================================================
    // Dangling References
    // ============================================================
    /*
    A dangling reference is a pointer to a memory address that has been deallocated. This creates a bug in other programming languages
    Rust prevents dangling references at compile time
    This function does not compile because city is dropped when the function ends
    fn create_city() -> &String {
        let city: String = String::from("New York");
        &city
    }
    */

    // ============================================================
    // Ownership with Arrays and Tuples
    // ============================================================
    let registrations: [bool; 3] = [true, false, true];
    let first_registration = registrations[0]; // Bool implements the Copy trait, so the value is copied from the array
    println!("First registration: {}", first_registration);
    println!("Registrations: {:?}", registrations);

    let languages: [String; 3] = [String::from("Rust"), String::from("Go"), String::from("Python")];
    let first_language = &languages[0]; // String does not implement the Copy trait, so we borrow the value instead of moving it
    println!("First language: {}", first_language);
    println!("Languages: {:?}", languages);
    // The same rules applies to tuples
   
}
