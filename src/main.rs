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

    //Compound types

    // Arrays
    let arr: [u8; 3] = [1, 2, 3];

    //print structure of the array
    println!("arr: {:?}", arr);

    //Tuples - a DS that can hold unlimited elements of different types
    let tuple: (u8, bool, f32) = (5, true, 2.1);
    let tuple2 = (3, 5);

    print!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple2);

    let (a, b, c) = tuple;

    //destructuring a tuple
    println!("first{}, second{}, third{}", a, b, c);

    // Function call
    println!("{}", is_even(4));

    //Mutability - needs to be explicitly defined
    let mut num = 5;
    num = 3;
    println!("{}", num);

    //Slice
    let arr = [1, 2, 3, 4, 5, 6];
    let slice = &arr[1..6]; // [1,2,3] required when don't know the length of the array
    borrowing_slice(arr, slice)
}

fn borrowing_slice(arr: [u8; 6], slice: &[u8]) {
    println!("Array : {:?}", arr);
    println!("Slice : {:?}", slice);
    println!("Lenght : {:?}", slice.len());
    println!("{}, {}", slice[0], slice[1]);
}

pub fn is_even(num: u8) -> bool {
    let digit = num % 2;
    digit == 0
}
