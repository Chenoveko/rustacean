#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

// ============================================================
// Defining Enums
// ============================================================
// An enum is a type that represents a set of possible values. Each possible value is called a variant 
// We use PascalCse to define enums and it's variants

#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

#[derive(Debug)]
struct Card {
    rank: String,
    suit: CardSuit
}

// ============================================================
// Enum with Associated Values
// ============================================================

#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    Paypal(String, String)
}

// ============================================================
// Struct Variant
// ============================================================
// A struct variant stores associated data in fields rather than by position. Each piece of data has an associated name

#[derive(Debug)]
enum PaymentMethod {
    CreditCard(String),
    DebitCard(String),
    Paypal { username: String, password: String },
    Cash
}

// ============================================================
// Nesting Enums in Enums
// ============================================================
#[derive(Debug)]
enum Beans {
    Pinto,
    Black
}

#[derive(Debug)]
enum Meat {
    Chicken,
    Steak
}

#[derive(Debug)]
enum RestaurtantItem {
    Burrito { meat: Meat, beans: Beans},
    Bowl(Meat),
    VeganPlate
}

fn main() {
    // ============================================================ 
    // Instance Enums
    // ============================================================
    let first_card: CardSuit = CardSuit::Hearts;
    let mut second_card: CardSuit = CardSuit::Spades;
    second_card = CardSuit::Clubs;
    println!("{:?}", second_card);

    // ============================================================ 
    // Instance Enums with Associated Values
    // ============================================================
    let visa: PaymentMethodType = PaymentMethodType::CreditCard(String::from("0034-4582"));
    let mastercard: PaymentMethodType = PaymentMethodType::DebitCard(String::from("1234-5678"));
    let my_paypal: PaymentMethodType = PaymentMethodType::Paypal(String::from("bob@gmail.com"), String::from("password"));
    println!("{:?} {:?} {:?}", visa, mastercard, my_paypal);

    // ============================================================ 
    // Instance Struct Variants
    // ============================================================
    let other_paypal: PaymentMethod = PaymentMethod::Paypal{
        username: String::from("bob@gmail.com"), 
        password: String::from("1234")
    };
    println!("{:?}", other_paypal);

    // ============================================================ 
    // Instance Enums Nested
    // ============================================================
    let lunch: RestaurtantItem = RestaurtantItem::Burrito{ meat: Meat::Steak, beans: Beans::Pinto };
    let dinner: RestaurtantItem = RestaurtantItem::Bowl(Meat::Chicken);
    println!("Lunch: {:?}", lunch);
    println!("Dinner: {:?}", dinner);

    // ============================================================ 
    // Match with Enums
    // ============================================================
    enum OperatingSytem {
        Windows,
        MacOS,
        Linux
    }

    fn years_since_release(os: OperatingSytem) -> u32 {
        match os {
            OperatingSytem::Windows => 39,
            OperatingSytem::MacOS => {
                println!("Steve Jobs OS");
                23
            }
            OperatingSytem::Linux => 34
        }
    }

    let my_computer = OperatingSytem::MacOS;
    let age: u32 = years_since_release(my_computer);
    println!("My computer's os is {} years old", age);

    enum LaundryCycle {
        Cold,
        Hot { temperature: u32 },
        Delicate(String)
    }

    fn wash_laundry(cycle: LaundryCycle) {
        match cycle {
            LaundryCycle::Cold => println!("Running the laundry with cold temperature"),
            LaundryCycle::Hot { temperature } => println!("Running the laundry with a temperature of {} degrees", temperature),
            LaundryCycle::Delicate(fabric_type) => println!("Running the laundry with a delicate cyle for {}", fabric_type)
        }
    }

    wash_laundry(LaundryCycle::Cold);
    wash_laundry(LaundryCycle::Hot { temperature: 100 });
    wash_laundry(LaundryCycle::Delicate(String::from("Silk")));

    // ============================================================
    // Methods on Enums
    // ============================================================
    impl LaundryCycle {
        fn wash_laundry(&self) {
            match self {
                LaundryCycle::Cold => println!("Running the laundry with cold temperature"),
                LaundryCycle::Hot { temperature } => println!("Running the laundry with a temperature of {} degrees", temperature),
                LaundryCycle::Delicate(fabric_type) => println!("Running the laundry with a delicate cyle for {}", fabric_type)
            }
        }
    }
    let my_laundry_cycle: LaundryCycle = LaundryCycle::Delicate(String::from("Silk"));
    my_laundry_cycle.wash_laundry();

    #[derive(Debug)]
    enum OnlineOrderStatus {
        Ordered,
        Packed,
        Shipped,
        Delivered
    }

    impl OnlineOrderStatus{
        fn check(&self) {
            // match with multiple values
            match self {
                OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed  => println!("Your item is being prepped for shipment"),
                OnlineOrderStatus::Delivered => println!("Your item has been delivered"),
                other_status => println!("Your item is {:?}", other_status)
            }
        }   
    }

    OnlineOrderStatus::Ordered.check();
    OnlineOrderStatus::Packed.check();
    OnlineOrderStatus::Shipped.check();
    OnlineOrderStatus::Delivered.check();

    enum Milk {
        LowFat(u32),
        Whole
    }

    impl Milk{
        fn drink(&self) {
            // match with exact values
            match self {
                Milk::LowFat(2) => println!("Delicious, 2% milk is my favourite!"),
                Milk::LowFat(percent) => println!("You've got the lowfat {} percent version!", percent),
                Milk::Whole => println!("Whole milk 🐂")
            }
        }   
    }

    Milk::LowFat(1).drink();
    Milk::LowFat(2).drink();
    Milk::Whole.drink();

    // ============================================================
    // if let construct
    // ============================================================
    // The if let construct combines an if statement with a variable declaration
    enum GoatMilk {
        LowFat(u32),
        Whole,
        NonDairy { kind: String }
    }

    let my_beverage: GoatMilk = GoatMilk::Whole;
    if let GoatMilk::Whole = my_beverage {
        println!("You have whole goat milk 🐐")
    }

    let my_beverage: GoatMilk = GoatMilk::LowFat(2);
    if let GoatMilk::LowFat(percent) = my_beverage {
        println!("You've got the lowfat goat milk {} percent version!", percent)
    }

    let my_beverage: GoatMilk = GoatMilk::NonDairy{
        kind: String::from("Oat")
    };
    if let GoatMilk::NonDairy { kind } = my_beverage {
        println!("Your beverage is {} milk", kind)
    } else {
        println!("Other variant")
    }

    // ============================================================
    // let else construct
    // ============================================================
    let my_beverage: GoatMilk = GoatMilk::Whole;

    let GoatMilk::LowFat(percent) = my_beverage else {
        println!("You do not have the lowfat milk");
        return;
    };

}
