#![allow(unused_variables)]
#![allow(dead_code)]

// ============================================================
// Define a Struct
// ============================================================
/*
A struct (structure) is a container for related pieces of data. Similar to an object in POO
We use PascalCase for Structs names and snake_case for fields in the struct
Rust has 3 kinds of structs:
    - Named Field Structs (Most of the time)
    - Tuple-Like Structs
    - Unit-Like Structs
*/
#[derive(Debug)]
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool
}

// ============================================================
// Defining Struct Methods
// ============================================================
#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32
}

impl TaylorSwiftSong {
    /*
    There are 4 possible ways to pass self:
        - Immutable struct value (self parameter takes ownership) -> display_song_info
        - Mutable struct value (self parameter takes ownership, has permission to mutate) -> double_length
        - Immutable reference to the struct instance (no ownership moved) -> display_song_info_ref
        - Mutable reference to the struct instance (no ownership moved, has permission to mutate) -> double_length_ref
    `Self` is an alias for the type being implemented.
    In this case, `Self` refers to `TaylorSwiftSong`.
    Therefore, these 3 method definitions are equivalent:
        fn display_song_info(self: TaylorSwiftSong) {}
        fn display_song_info(self: Self) {}
        fn display_song_info(self) {}
    */
    fn display_song_info(self) {
        println!("Title: {}", self.title);
        println!("Release year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }

    fn double_length(mut self) {
        let previous_length = self.duration_secs;
        self.duration_secs *= 2;
        println!("Previous length: {} seconds. New length: {} seconds", previous_length, self.duration_secs);
    }

    fn display_song_info_ref(&self) {
        println!("Title: {}", self.title);
        println!("Release year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }

    fn double_length_ref(&mut self) {
        self.duration_secs *= 2;
    }

    // ============================================================
    // Methods with Multiple Parameters
    // ============================================================
    fn is_longer_than(&self, other_song: &Self) -> bool {
        self.duration_secs > other_song.duration_secs
    }

    // ============================================================
    // Calling Methods from Other Methods
    // ============================================================
    fn years_since_release(&self) -> u32 {
        2026 - self.release_year
    }

    fn display_year_since_release(&self) {
        println!("Years since release: {}", self.years_since_release());
    }

    // ============================================================
    // Associated Functions
    // ============================================================
    /* Associated functions are functions that are attached to a type
    Examples:
        - String::from()
        - String::new()
    We often use associated functions for constructors. A constructor is a function that return a new instance of a type
    */
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        Self { title, release_year, duration_secs }
    }
}

// ============================================================
// Multiple impl Blocks
// ============================================================
// We can have multiple impl blocks, is totally valid
impl TaylorSwiftSong {
    fn display_title(&self) {
        println!("Title: {}", self.title);
    }
}

// ============================================================
// Builder Pattern
// ============================================================
// A design pattern is a recommended way to write or structure code to solve specific problems
#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self { cpu, memory, hard_drive_capacity }
    }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self{
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }

}

// ============================================================
// Tuple Struct
// ============================================================
// A tuple struct is a struct that assigns each piece of data an order in line rather than a name
struct ShortDuration(u32, u32); // Hours, Minutes
struct LongDuration(u32, u32); // Years, Months

// ============================================================
// Unit-Like Struct
// ============================================================
struct Empty;

fn main() {
    // ============================================================
    // Struct Instance
    // ============================================================
    // An instance is the concrete value made from a type
    let mocha: Coffee = Coffee {
        name: String::from("Mocha"),
        price: 4.95,
        is_hot: true
    };
    //  A struct is the owner of it's fields and each field is the owner of it's corresponding value
    println!("{:?}", mocha);
    println!("Coffee name: {}", mocha.name);
    println!("Coffee price: ${:.2}", mocha.price);
    println!("Is hot: {}", mocha.is_hot);

    // ============================================================
    // Overwrite Struct Fields
    // ============================================================
    let mut latte: Coffee = Coffee {
        name: String::from("Latte"),
        price: 3.20,
        is_hot: false
    };

    latte.name = String::from("Vegan latte");
    latte.price = 3.10;
    latte.is_hot = true;
    println!("Latte modified: {:?}", latte);

    // ============================================================
    // Create Structs in a Function
    // ============================================================
    fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
        Coffee {
            name: name,
            price: price,
            is_hot: is_hot
        }
    }
    let f_coffee: Coffee = make_coffee(String::from("f Coffee"), 2.1, true);
    println!("Function coffee: {:?}", f_coffee);

    // ============================================================
    // Struct Field Initialization Shorthand Syntax
    // ============================================================
    // When the struct field name match the function parameter name
    fn make_coffee_short(name: String, price: f64, is_hot: bool) -> Coffee {
        Coffee { name, price, is_hot}
    }
    let fs_coffee: Coffee = make_coffee_short(String::from("fs Coffee"), 2.2, true);
    println!("Short Function coffee: {:?}", fs_coffee);
    // Same shorthand syntax applies for variables
    let name: String = String::from("Macciato");
    let price: f64 = 3.99;
    let is_hot: bool = true;
    let macciato: Coffee = Coffee { name, price, is_hot};
    println!("Macciato coffee: {:?}", macciato);

    // ============================================================
    // Struct Update Syntax
    // ============================================================
    // Struct update syntax creates a new struct instance by assigning a new value to specific fields and reusing the remaining fields from an existing struct instance
    let caramel_macciato: Coffee = Coffee {
        name: String::from("Caramel Macciato"),
        ..macciato
    };
    println!("Caramel Macciato coffee: {:?}", caramel_macciato);
    // Be careful with fields such as String because String does not implement the Copy trait. Fields that are not explicitly assigned may be moved from the original struct when using struct update syntax
    // use .clone() with strings

    // ============================================================
    // Passing Structs into a Function
    // ============================================================
    /* 4 possible ways
        - Receive the struct as an immutable value. The function takes ownership -> drink_coffee1
        - Receive the struct as a mutable value. The function takes ownership -> drink_coffee2
        - Receive the struct as an immutable reference. The function does not takes ownership -> drink_coffee3
        - Receive the struct as a mutable reference. The function does not takes ownership -> drink_coffee4
    */
    let mocha: Coffee = Coffee {
        name: String::from("Mocha"),
        price: 4.95,
        is_hot: true
    };
    fn drink_coffee1(coffee: Coffee) {
        println!("Drinking my delicious {} coffee", coffee.name);
    }
    drink_coffee1(mocha);

    let mocha: Coffee = Coffee {
        name: String::from("Mocha"),
        price: 4.95,
        is_hot: true
    };
    fn drink_coffee2(mut coffee: Coffee) {
        coffee.is_hot = false;
        println!("Drinking my delicious cold {} coffee", coffee.name);
    }
    drink_coffee2(mocha);

    let mocha: Coffee = Coffee {
        name: String::from("Mocha"),
        price: 4.95,
        is_hot: true
    };
    fn drink_coffee3(coffee: &Coffee) {
        println!("Drinking my delicious {} coffee from reference", coffee.name);
    }
    drink_coffee3(&mocha);

    let mut mocha: Coffee = Coffee {
        name: String::from("Mocha"),
        price: 4.95,
        is_hot: true
    };
    fn drink_coffee4(coffee: &mut Coffee) {
        coffee.is_hot = false;
        println!("Drinking my delicious cold {} coffee from mut reference", coffee.name);
    }
    drink_coffee4(&mut mocha);

    // ============================================================
    // Invoke Structs Methods
    // ============================================================
    let song: TaylorSwiftSong= TaylorSwiftSong { title: String::from("Blank Space"), release_year: 2014, duration_secs: 231 };
    song.display_song_info();

    let song: TaylorSwiftSong= TaylorSwiftSong { title: String::from("Blank Space"), release_year: 2014, duration_secs: 231 };
    song.double_length();

    let song: TaylorSwiftSong= TaylorSwiftSong { title: String::from("Blank Space"), release_year: 2014, duration_secs: 231 };
    song.display_song_info_ref();
    println!("{:?}", song);

    let mut song: TaylorSwiftSong= TaylorSwiftSong { title: String::from("Blank Space"), release_year: 2014, duration_secs: 231 };
    song.double_length_ref();
    println!("{:?}", song);

    let song: TaylorSwiftSong= TaylorSwiftSong { title: String::from("Blank Space"), release_year: 2014, duration_secs: 231 };
    let other_song: TaylorSwiftSong= TaylorSwiftSong { title: String::from("Opalite"), release_year: 2016, duration_secs: 320 };
    println!("{}", song.is_longer_than(&other_song));
    song.display_year_since_release();

    // ============================================================
    // Invoke Constructor
    // ============================================================
    let blank_space: TaylorSwiftSong = TaylorSwiftSong::new(String::from("Blank Space"), 2014, 231);
    println!("{:?}", blank_space);

    // ============================================================
    // Builder Pattern
    // ============================================================
    let mut laptop: Computer = Computer::new(String::from("M3 Max"), 64, 128);
    laptop  
        .upgrade_cpu(String::from("M4 Max"))
        .upgrade_memory(86)
        .upgrade_hard_drive_capacity(256);
    println!("My upgrade laptop {:?}", laptop);

    // ============================================================
    // Tuple Struct Instance
    // ============================================================
    let work_shift: ShortDuration = ShortDuration(8, 0);
    println!("{} hours {} minutes", work_shift.0, work_shift.1);
    let era: LongDuration = LongDuration(5, 3);
    println!("{} years {} months", era.0, era.1);

    // ============================================================
    // Unit-Like Structs Instance
    // ============================================================
    // A unit is an empty tuple, a tuple without values
    let unit: () = ();
    let my_empty_struct: Empty = Empty;

}
