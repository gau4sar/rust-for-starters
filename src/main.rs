fn main() {
    println!("Hello, world!");

    // Primitive types

    let unsignedInt: u8 = 10;
    let signedInt: i8 = -10;
    let float: f32 = 1.2;

    println!(
        "unsignedInt: {}, signedInt: {}, float: {}",
        unsignedInt, signedInt, float
    );

    let letter = "ch12123";
    let emoji = "\u{1F600}";

    println!("letter: {}, emoji: {}", letter, emoji);

    let isBoolean = true || false;
    println!("isBoolean: {}", isBoolean);
}
