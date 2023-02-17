use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let c = 'z';
    let z: char = 'Z';
    let puss = '😻';

    let tup: (i32, f64, bool) = (-32, 45.54322, false);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let second_value = tup.1; // 45.54322 (why different of array)

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    let third = arr[2];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
