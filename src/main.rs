// use std::io;

fn main() {
    println!("Fibonacci: {}", fibonacci(6)); // Should be 13.

    println!("Fahrenheit -> Celsius: {}", fahrenheit_to_celsius(80));
    println!("Celsius -> Fahrenheit: {}", celsius_to_fahrenheit(25));

    twelve_days_of_christmas();
    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line!");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");
}

fn fibonacci(count: i32) -> i32 {
    do_fibonacci(count, 0, 1)
}

fn do_fibonacci(count: i32, prev: i32, current: i32) -> i32 {
    match count {
        0 => current,
        _ => do_fibonacci(count - 1, current, prev + current),
    }
}

// C = (F - 32) * 5/9
fn fahrenheit_to_celsius(temperature: i32) -> i32 {
    (temperature - 32) * 5 / 9
}

// C = (F - 32) * (5/9)
// (C / (5/9)) = F - 32
// (C / (5/9)) + 32 = F
fn celsius_to_fahrenheit(temperature: i32) -> i32 {
    (temperature * 9 / 5) + 32
}

fn number_to_count<'a>(number: usize) -> &'a str {
    match number {
        0 => "A",
        1 => "Two",
        2 => "Three",
        3 => "Four",
        4 => "Five",
        5 => "Six",
        6 => "Seven",
        7 => "Eight",
        8 => "Nine",
        9 => "Ten",
        10 => "Eleven",
        11 => "Twelve",
        _ => panic!("Use a smaller number!"),
    }
}

fn number_to_day<'a>(number: usize) -> &'a str {
    match number {
        0 => "first",
        1 => "second",
        2 => "third",
        3 => "forth",
        4 => "fifth",
        5 => "sixth",
        6 => "seventh",
        7 => "eighth",
        8 => "nineth",
        9 => "tenth",
        10 => "eleventh",
        11 => "twelfth",
        _ => panic!("Use a smaller number!"),
    }
}

fn twelve_days_of_christmas() {
    let presents = [
        "partridge",
        "turtle doves",
        "french hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    for day in 0..12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            number_to_day(day)
        );

        for present_for_day in (0..=day).rev() {
            let suffix = if present_for_day == 1 { ", and" } else { "" };

            println!(
                "{} {}{}",
                number_to_count(present_for_day),
                presents[present_for_day],
                suffix
            );
        }

        println!("");
    }
}
