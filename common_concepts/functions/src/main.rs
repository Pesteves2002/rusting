fn main() {
    say_hi();
    print_plus_one(5);
    println!("The answer is {}", return_function(46));
}

fn say_hi() {
    println!("Hi!");
}

fn print_plus_one(x: i32) {
    println!("x + 1 = {}", x + 1);
}

fn return_function(x: i32) -> i32 {
    x * 10
}
