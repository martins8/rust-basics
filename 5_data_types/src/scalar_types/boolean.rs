pub fn boolean() {
    let t = true;
    let f: bool = false;

    let is_rust_fun = t;
    if is_rust_fun && !f {
        println!("Rust is fun!");
    } else {
        println!("Rust is not fun!");
    }
}
