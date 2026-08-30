pub fn integer() {
    let x: i8 = -10;
    let y: u8 = 10;
    /*
     * i8: signed 8-bit integer
     * u8: unsigned 8-bit integer
     * i16: signed 16-bit integer
     * u16: unsigned 16-bit integer
     * i32: signed 32-bit integer
     * u32: unsigned 32-bit integer
     * i64: signed 64-bit integer
     * u64: unsigned 64-bit integer
     * i128: signed 128-bit integer
     * u128: unsigned 128-bit integer
     * isize: signed size integer
     * usize: unsigned size integer
     */
    println!("x: {x}, y: {y}");
}

pub fn float() {
    let x: f32 = 3.14;
    let y = 6.0; //default f64
    println!("x: {x}, y: {y}");
}

pub fn numeric_operations() {
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;
    println!(
        "sum: {sum}, difference: {difference}, product: {product}, quotient: {quotient}, truncated: {truncated}, remainder: {remainder}"
    );
}
