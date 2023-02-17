fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&mut s1);

    println!("The length of '{}' is {}.", s1, len);

    let r1 = &mut s1;

    println!("r1: {}", r1);

    let r2 = &s1;
    let r3 = &s1;

    println!("r2: {}", r2);
    println!("r3: {}", r3);

    let r4 = &mut s1;
    println!("r4: {}", r4); // no problem

    let s5 = no_dangle();
    println!("s5: {}", s5);
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
