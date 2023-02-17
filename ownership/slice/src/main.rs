fn main() {
    let s = String::from("OBAMA HAS YOU COVERED!");

    let first_word = first_word(&s);

    println!("1: {}", first_word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
