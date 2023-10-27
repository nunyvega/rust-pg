// ============== SECTION: Basic Printing and Variables ==============
fn basic_printing_and_variables() {
    println!("Hello, world!");

    let stack_1 = 32;
    let stack_2 = stack_1; // The value of `stack_1` is copied into `stack_2`

    // We now have two values we can work with
    println!("{}", stack_1);
    println!("{}", stack_2);
}

// ============== SECTION: Functions and Ownership ==============
fn functions_and_ownership() {
    // When we have a function that return a value, the ownership of that value is passed to the caller.
    fn abc() -> String {
        "abc".to_string()
    }

    let letters = abc(); // The value created in `abc()` is now owned by `letters`

    // When we have a function that passes a value through it, it can be thought of as temporarily taking ownership of that value until the function call has completed.
    fn print_through(s: String) -> String {
        s
    }

    let _finished = print_through(letters); // letters has been moved into finished
    let juan = abc();

    print!("{}", juan);

    let pi = 3.141592;
    let funny_number = &pi;
    println!("{}", funny_number);
}

// ============== SECTION: References and Mutability ==============
fn references_and_mutability() {
    let mut year = 3020;
    let y = &mut year;

    let _ = *y + 10;

    println!("The year is {}.", year);

    let planet = "Earth";
    let earth = &&&&planet;

    assert_eq!("EARTH", earth.to_uppercase());

    let starship: Option<String> = Some("Omaha".to_string());

    match starship {
        // The ref keyword is used here to take a reference of the value inside Some
        // rather than taking ownership of the value.
        // This way, starship retains ownership of its value.
        Some(ref name) => println!("{}", name),
        None => {}
    }

    // Without the use of the `ref` keyword above, this next line would not compile:
    println!("{:?}", starship);
    // {:?} is used for formatting, which means it will print the debug representation of starship.
    println!("{:#?}", starship);
    // {:#?} also exists for pretty print

    let val = "reciprocal";

    let ref r1 = val;
    let r2 = &val;

    assert_eq!(r1, r2);

    let s = String::from("hello world");

    let hello = &s[0..5]; // Slices are a reference to a range of elements in a collection
    let world = &s[6..11];
    println!("{hello}{world}");

    // This is a closure
    let double = |d| d * 2;

    // This is the outcome of calling the closure
    let var = double(10);
    // This will re-assign the value of var
    let doubled_var = var;
    println!("{}", doubled_var);

    //Shadowing an inmutable variable and making it mutable
    let number = 10;
    let mut number = number + 10;
    number += 10;
    println!("{number}"); // prints "23"

    fn question(s: &mut String) {
        s.pop();
        s.push('?');
    }

    let mut sentence = String::from("I am.");
    question(&mut sentence);
}

// ============== SECTION: Macros ==============
fn macro_section() {
    println!("Macros section");

    // Calls ending with `!` are macros
    // Basically before the code is compiled, the macros used are replaced by the actual code.
    // is a macro because it formats the string on compiletime, it calls format!
    println!("Macros are a powerful feature in Rust!");

    let number = 10;
    if number <= 5 {
        todo!("we will handle this outcome soon.") // another macro
    } else if number > 5 {
        //unimplemented!("we might do something here eventually") // and another, commented because it breaks the build
    } else {
        // unreachable!() // and another
    }
}

// ============== SECTION: Attributes ==============
fn attributes_section() {
    //Attributes are also macros attributes example

    #[derive(Debug)]
    struct Wow;

    let wow = Wow;

    // Calls ending with `!` are macros
    // Basically before the code is compiled, the macros used are replaced by the actual code.
    // is a macro because it formats the string on compiletime, it calls format!
    println!("{wow:?} that is convenient!");

    // #[cfg] is used to tell the compiler whether or not to compile
    // the following code based on a condition.
    #[cfg(target_os = "linux")]
    fn are_you_on_linux() {
        println!("You are running linux!");
    }

    #[cfg(not(target_os = "linux"))]
    fn are_you_on_linux() {
        println!("You are *not* running linux!");
    }

    are_you_on_linux();

    let number = 12;
    let you = "us";
    let i = "us";

    assert!(you == i); // assert is also a macro!
    assert_eq!(i, you);
    assert_ne!(number, you.len());
}

// ============== SECTION: Traits ==============
fn traits_section() {
    trait Golf {
        const BIRDIE: i32 = -1;
    }

    struct Caddy;

    impl Golf for Caddy {}

    println!("{}", Caddy::BIRDIE);
}

// ============== SECTION: Control Flow ==============
fn control_flow_section() {
    let number = 10;
    if number <= 5 {
        todo!("we will handle this outcome soon.") // another macro
    } else if number > 5 {
        //unimplemented!("we might do something here eventually")
        // and another, commented because it breaks the break
    } else {
        unreachable!() // and another
    }
    // todo!, unimplemented! and unreachable! all panic! when called.
    // panic!("we should use panics sparingly."); // and another!
}

// ============== SECTION: If Let ==============
fn if_let_section() {
    println!("== If Let Section ==");

    // Using if let with Option
    let some_option: Option<i32> = Some(10);

    if let Some(value) = some_option {
        println!("The value inside the option is: {}", value);
    } else {
        println!("The option is None");
    }

    // Using if let with custom Enum
    enum Weather {
        _Sunny,
        Rainy(String),
        _Cloudy,
    }

    let today = Weather::Rainy(String::from("Heavy"));

    if let Weather::Rainy(description) = today {
        println!("The weather today is rainy with: {}", description);
    } else {
        println!("It's not rainy today");
    }
}

// ============== Main Function Calling Each Section ==============
fn main() {
    basic_printing_and_variables();
    functions_and_ownership();
    references_and_mutability();
    macro_section();
    traits_section();
    control_flow_section();
    attributes_section();
    if_let_section();
}
