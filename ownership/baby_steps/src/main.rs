fn main() {
    let mut s = String::from("hello"); // s comes into scope

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    let s1 = String::from("hello");
    let s2 = s1;

    // DOES NOT WORK SINCE s1 IS MOVED TO s2
    // println!("{}, world!", s1);

    println!("{}, world!", s2);

    // More expensive to copy the data on the heap
    let c1 = String::from("hello");
    let c2 = c1.clone();

    println!("c1 = {}, c2 = {}", c1, c2);

    takes_ownership(s2); // s2's value moves into the function...
                         // ... and so is no longer valid here

    // println!("{}", s2); // DOES NOT WORK SINCE s2 IS MOVED TO takes_ownership

    let x = 5; // x comes into scope

    makes_copy(x); // 5 would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use 5 afterward
    println!("{}", x); // This will print `5`

    let s3 = gives_ownership(); // gives_ownership moves its return
                                // value into s3

    let s3 = takes_and_gives_back(s3); // s3 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s4

    println!("{}", s3); // This will print `yours`
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
