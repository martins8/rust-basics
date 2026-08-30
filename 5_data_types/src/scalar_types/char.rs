pub fn char() {
    // needs to be a single ' quote, because " is a string literal
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);
}
