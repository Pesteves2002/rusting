fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let value = if number < 4 { 3 } else { 4 };
    println!("The value of value is: {}", value);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 5;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let arr = ['o', 'b', 'a', 'm', 'a'];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", arr[index]);
        index += 1;
    }

    for element in arr {
        println!("Faster: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    christmas_carol();
}

fn farn_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// slow recursive fibonacci
fn n_th_fib_num(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    n_th_fib_num(n - 1) + n_th_fib_num(n - 2)
}

fn christmas_carol() {
    // the twelve days of christmas
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    // the gifts
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for i in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            days[i]
        );
        for j in (0..i + 1).rev() {
            if i == 0 {
                println!("{}.", gifts[j]);
            } else if j == 0 {
                println!("and {}.", gifts[j]);
            } else {
                println!("{}", gifts[j]);
            }
        }
    }
}
