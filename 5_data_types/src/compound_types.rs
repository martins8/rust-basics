pub fn compound_types() {
    //tuple
    let tup: (i32, f64, bool) = (500, 6.4, true);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 255);
    let five_hundred = x.0;
    println!("five_hundred: {}", five_hundred);

    let mut x: (i32, f64, u8) = (500, 6.4, 255);
    x.0 = 1000;
    println!("x: {}, y: {}, z: {}", x.0, x.1, x.2);

    /* array is a fixed-size list of elements of the same type
     * and it lives on the stack memory
     * it is more useful when we know the size of the list
     * and we don't need to resize it */
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array: {:?}", array);
}
