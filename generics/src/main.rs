// ============================================================
// Generics in Structs
// ============================================================

#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

// ============================================================
// Generics in impl Blocks
// ============================================================
//With this syntax we declare methods for any type T

impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

// ============================================================
// Implementation for a Specific Type
// ============================================================
// We can hardcode a specific type to declare methodsfor only that specific type

impl TreasureChest<i32> {
    fn double_value(&self) -> i32 {
        self.treasure * 2
    }
}

// ============================================================
// Generics in Enums
// ============================================================

#[derive(Debug)]
enum Cheesesteak<T> {
    Plain,
    Topping(T)
}

fn main() {
    // ============================================================
    // Intro to Generics
    // ============================================================
    // A generic is a type argument. It's a placeholder for a future type
    fn identity<T>(value: T) -> T {
        value
    }
    println!("identity: {}", identity("Hello World!"));
    println!("identity: {}", identity(5));
    println!("identity: {}", identity(5.0));
    println!("identity: {}", identity::<i8>(5)); // Use the turbofish operator "::<type>" to customize the type of the generic 

    // ============================================================
    // Multiple Generics
    // ============================================================
    fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
        (first, second)
    }
    println!("make_tuple: {:?}", make_tuple(5.0, "Tuple"));

    // ============================================================
    // Instance Structs with Generic Fields
    // ============================================================
    let gold_chest: TreasureChest<&str> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold",
    };
    let silver_chest: TreasureChest<String> = TreasureChest {
        captain: String::from("Bloodsail"),
        treasure: String::from("Silver"),
    };
    let black_pearl: TreasureChest<i32> = TreasureChest {
        captain: String::from("Bloodsail"),
        treasure: 134,
    };
    println!("gold_chest: {:?}", gold_chest);
    println!("silver_chest: {:?}", silver_chest);
    println!("black_pearl: {:?}", black_pearl);

    // ============================================================
    // Invoke Structs Methods with Generics
    // ============================================================
    println!("silver_chest capital captain: {}", silver_chest.capital_captain());
    println!("black_pearl double value: {}", black_pearl.double_value());

    // ============================================================
    // Instance Enums with Generic Fields
    // ============================================================
    let mushroom: Cheesesteak<&str> = Cheesesteak::Topping::<&str>("mushroom"); // THe turbofish oeprator is optional
    let onion: Cheesesteak<String> = Cheesesteak::Topping("onions".to_string()); 
    let plain: Cheesesteak<&str> = Cheesesteak::Plain("plain")

    println!("mushroom: {:?}", mushroom);
    println!("onion: {:?}", onion);
    println!("plain: {:?}", plain);
}
