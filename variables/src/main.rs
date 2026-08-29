const THREE_HOURS_IN_SECONDS: u64 = 10800;

fn main() {
    mutable();
    shadowing();
}

fn mutable() {
    let mut x = 5;
    println!("THe value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = x + THREE_HOURS_IN_SECONDS;
    println!("The value of y is: {y}");
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("THe value of x in the inner scope is {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "    ";
    println!("spa{spaces}ces");
    let spaces = spaces.len();
    println!("spaces: {spaces}");

    /*
    - if try to catch .len() on a mutable variable
    - it will not work because spaces is a string literal and cannot be mutated

    let mut spaces = "    ";
    spaces = spaces.len();

    DOESN'T COMPILE
    */
}
