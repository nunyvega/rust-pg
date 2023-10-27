// ============== SECTION: Basic Printing and Variables ==============
fn basic_printing_and_variables() {
    println!("== Basic Printing and Variables Section ==");
    println!("Hello, world!");

    let stack_1 = 32;
    let stack_2 = stack_1; // The value of `stack_1` is copied into `stack_2`

    // We now have two values we can work with
    println!("{}", stack_1);
    println!("{}", stack_2);
}

// ============== SECTION: Functions and Ownership ==============
fn functions_and_ownership() {
    println!("== Functions and Ownership Section ==");
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
    println!("== References and Mutability Section ==");
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
    println!("== Macros Section ==");

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
    println!("== Attributes Section ==");
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
    println!("== Traits Section ==");
    trait Golf {
        const BIRDIE: i32 = -1;
    }

    struct Caddy;

    impl Golf for Caddy {}

    println!("{}", Caddy::BIRDIE);
}

// ============== SECTION: Control Flow ==============
fn control_flow_section() {
    println!("== Control Flow Section ==");
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

// ============== SECTION: Match ==============
fn match_section() {
    println!("== Match Section ==");

    // Using match with integers
    let number = 1;

    // Match is exhaustive, so we need to cover all possible cases
    match number {
        1 => println!("The number is one."),
        2 => println!("The number is two."),
        3..=20 => println!("The number is between 3 and 20"),
        _ => println!("The number is neither one nor two."),
    }

    // Using match with Option
    let some_option: Option<i32> = Some(10);

    match some_option {
        Some(value) => println!("The value inside the option is: {}", value),
        None => println!("The option is None"),
    }

    // Using match with custom Enum
    enum Weather {
        _Sunny,
        Rainy(String),
        _Cloudy,
    }

    let today = Weather::Rainy(String::from("Heavy"));

    match today {
        Weather::_Sunny => println!("The weather today is sunny."),
        Weather::Rainy(description) => println!("The weather today is rainy with: {}", description),
        Weather::_Cloudy => println!("The weather today is cloudy."),
    }
}

// ============== SECTION: Loops ==============
fn loops_section() {
    println!("== Loops Section ==");

    // Loop - Infinite loop, can be terminated using a condition within the loop
    let mut count = 0;
    loop {
        println!("This is an infinite loop iteration {}", count);
        count += 1;
        if count == 3 {
            break;
        }
    }

    // Loop Labels - Named loops that allow breaking or continuing outer loops
    'outer: loop {
        println!("Outer loop");
        loop {
            println!("Inner loop");
            break 'outer; // This will break the outer loop
        }
    }

    // While Loop - Continues as long as the condition is true
    let mut number = 5;
    while number != 0 {
        println!("The number is {}", number);
        number -= 1;
    }

    // While Let - Continues as long as the pattern in the condition matches
    let mut optional_number = Some(3);
    while let Some(value) = optional_number {
        println!("The value inside the option is: {}", value);
        optional_number = if value != 0 { Some(value - 1) } else { None };
    }

    // For/In Loop - Iterates over elements of a collection
    for i in 1..4 {
        println!("This is a for/in loop iteration {}", i);
    }

    // For/In Loop with an iterator, like array's iter() method
    let array = [10, 20, 30];
    for element in array.iter() {
        println!("The value in the array is: {}", element);
    }
}

// ============== SECTION: Closures ==============
fn closures_section() {
    println!("== Closures Section ==");
    // Closures can capture values from the scope in which they're defined.
    // When we capture values from the environment, they are taken by reference.
    //If we want to take ownership of the values, we can use the move keyword.

    // A closure that takes no arguments and returns an i32.
    // The return type is inferred.
    let simple_closure = || 10;
    println!("Simple closure result: {}", simple_closure());

    // A closure that takes one argument and adds 1 to it.
    let add_one = |x: i32| x + 1;
    println!("Add one closure result: {}", add_one(5));

    // A closure that captures its environment, such as accessing a variable outside of its scope.
    let num = 5;
    let add_num = |x: i32| x + num;
    println!("Add num closure result: {}", add_num(10));

    // A closure with a move keyword, meaning it takes ownership of the values it uses from the environment.
    let subtract_num = move |x: i32| x - num;
    println!("Subtract num closure result: {}", subtract_num(10));
}

// ============== SECTION: Iterators ==============
fn iterators_section() {
    println!("== Iterators Section ==");

    // Using collect() to gather values into a collection
    let collected_vec: Vec<_> = (0..5).collect();
    println!("Collected into vector: {:?}", collected_vec);

    // Using map() to modify each element in the iterator
    let plus_one: Vec<_> = (0..5).map(|x| x + 1).collect();
    println!("Values after map (+1): {:?}", plus_one);

    // Using filter() to keep elements that satisfy a condition
    let even_numbers: Vec<_> = (0..5).filter(|x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even_numbers);

    // Using enumerate() to get the index along with the value
    for (index, value) in ('a'..='d').enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }

    // Using fold() as a reduce operation, to accumulate values
    let sum: i32 = (0..5).fold(0, |acc, x| acc + x);
    println!("Sum of numbers: {}", sum);

    // Using find() to get the first element that satisfies a condition
    let first_even = (0..5).find(|x| x % 2 == 0);
    println!("First even number: {:?}", first_even);
}

// ============== SECTION: Rust Primitives and Collections ==============
fn rust_primitives_section() {
    println!("== Rust Primitives and Collections Section ==");

    // Numeric Types (Integers, Floating-Point)
    // Integers include i8, i16, i32, i64, i128, u8, u16, u32, u64, u128
    // Floating-Point numbers include f32 and f64
    let int_value: i32 = 42;
    let float_value: f64 = 42.0;
    println!("Integer: {}, Floating-Point: {}", int_value, float_value);

    // Boolean
    // Represents a value that is either true or false
    let is_true: bool = true;
    println!("Boolean: {}", is_true);

    // Characters
    // Represents a Unicode character and is specified with a single quote
    let char_value: char = 'A';
    println!("Character: {}", char_value);

    // Strings
    // str (string slice) is immutable and &str is a reference to a string slice
    // String is a heap-allocated string type and is mutable
    let str_slice: &str = "hello";
    let string_obj: String = String::from("world");
    println!("Str slice: {}, String object: {}", str_slice, string_obj);

    // Arrays and Slices
    // Arrays have a fixed length at compile time.
    // If you want a collection that can grow or shrink in size, use a vector instead.
    // Slices are references to a sequence of elements in an array
    let array: [i32; 3] = [1, 2, 3];
    let slice: &[i32] = &array[1..3];
    println!("Array: {:?}, Slice: {:?}", array, slice);

    // Tuples
    // Ordered list of elements of potentially different types
    let tuple: (i32, &str) = (1, "one");
    println!("Tuple: {:?}", tuple);

    // Unit Type
    // Represents the absence of a value or information
    fn unit_function() {}
    println!("Unit value: {:?}", unit_function());

    // References & Pointers
    // References refer to the memory location of another variable
    // Raw pointers are similar to references but can be null or dangling
    let x = 10;
    let reference: &i32 = &x;
    let raw_pointer: *const i32 = &x;
    unsafe {
        println!("Reference: {}, Raw pointer: {}", reference, *raw_pointer);
    }

    // Homogeneous Collections
    // Vectors allow us to store multiple values of the same type
    let vector: Vec<i32> = vec![1, 2, 3];
    println!("Vector: {:?}", vector);

    // Other Collections include HashMaps, LinkedLists, BinaryHeaps, etc.
    // HashMaps store values based on a key
    use std::collections::HashMap;
    let mut hash_map: HashMap<&str, i32> = HashMap::new();
    hash_map.insert("one", 1);
    println!("HashMap: {:?}", hash_map);
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
    match_section();
    loops_section();
    closures_section();
    iterators_section();
    rust_primitives_section();
}
