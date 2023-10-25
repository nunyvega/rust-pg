fn main() {
    println!("Hello, world!");

    let stack_1 = 32;
    let stack_2 = stack_1; // The value of `stack_1` is copied into `stack_2`

    // We now have two values we can work with
    println!("{}", stack_1);
    println!("{stack_2}");

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

    let pi = 3.14159265359;
    let funny_number = &pi;

    println!("{funny_number}");

    let mut year = 3020;
    let y = &mut year;

    let _ = *y + 10;

    println!("The year is {year}.");

    let planet = "Earth";
    let earth = &&&&planet;

    assert_eq!("EARTH", earth.to_uppercase());
}
