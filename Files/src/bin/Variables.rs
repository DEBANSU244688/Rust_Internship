fn variables(){
    // Basic Types & Variables
    
    // Signed Integer
    let a: i8 = -10;
    let b: i16 = -1000;
    let c: i32 = -100000;
    let d: i64 = -10000000000;
    let e: i128 = -10000000000000000000;
    println!("Signed Integers:\na = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);

    // Unsigned Integer
    let f: u8 = 10;
    let g: u16 = 1000;
    let h: u32 = 100000;
    let i: u64 = 10000000000;
    let j: u128 = 10000000000000000000;
    println!("Unsigned Integers:\nf = {}, g = {}, h = {}, i = {}, j = {}", f, g, h, i, j);

    // Floating Point
    let k: f32 = 1.24;
    let l: f64 = 1.23456789;
    println!("Floating Points:\nk = {}, l = {}", k, l);

    // Boolean
    let is_true = true;
    let is_false = false;
    println!("Booleans:\nis_true = {}, is_false = {}", is_true, is_false);

    // Character
    let char_a: char = 'a';
    let char_ñ: char = 'ñ';
    let char_emoji: char = '😊';
    println!("Characters:\nchar_a = {}, char_ñ = {}, char_emoji = {}", char_a, char_ñ, char_emoji);

    // Character Bytes
    let char_a_bytes = "a".as_bytes();
    let char_ñ_bytes = "ñ".as_bytes();
    let char_emoji_bytes = "😊".as_bytes();
    println!("Character Bytes:\nchar_a_bytes = {:?}\n, char_ñ_bytes = {:?}\n, char_emoji_bytes = {:?}", char_a_bytes, char_ñ_bytes, char_emoji_bytes);
}

fn main() {
    println!("Basic Types & Variables:");
    variables();
}