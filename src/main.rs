use std::io;


// Q1
fn fahrenheit_to_celsius(fahrenheit: i16) -> f64 {
    (fahrenheit - 32) as f64 * 0.5556
}

// Q2
fn compute_fibonacci(fib: i16) -> i16 {
    if fib == 0 {
        return 0;
    }
    if fib == 1 {
        return 1;
    }
    compute_fibonacci(fib - 1) + compute_fibonacci(fib - 2)
}

// Q3
fn print_verse(sentence: (&str, &str), carol: String) -> String {
    let (number, phrase) = sentence;

    println!("On the {number} day of Christmas");
    println!("My good friends brought to me");
    println!("{phrase}\n{carol}");

    format!("{phrase}\n{carol}")
}


fn main() {
    println!("Welcome to this tutorial project used to experiment rust..");

    // 1. Convert temperatures between Fahrenheit and Celsius.
    println!("Please enter a temperature in Fahrenheit that needs to be converted to Celsius :");
    let mut fahrenheit = String::new();
    match io::stdin().read_line(&mut fahrenheit) {
        Ok(n) => {
            println!("{n} bytes read");
            println!("{fahrenheit}");
        }
        Err(error) => println!("error: {error}"),
    }

    let fahrenheit: i16 = fahrenheit.trim().parse().unwrap();
    let celcius: f64 = fahrenheit_to_celsius(fahrenheit);
    println!("Q1 : {fahrenheit}F is equal to {celcius:.1}°C !");


    // 2. Generate the nth Fibonacci number.
    println!("Please enter a Fibonacci number :");
    let mut fibonacci = String::new();
    match io::stdin().read_line(&mut fibonacci) {
        Ok(_n) => {
            println!("{fibonacci}");
        }
        Err(error) => println!("error: {error}"),
    }

    let fibonacci: i16 = fibonacci.trim().parse().unwrap();
    let fibonacci_result = compute_fibonacci(fibonacci - 1);

    println!("Q2 : {fibonacci}th fibonacci number is equivalent to {fibonacci_result}\n");


    // 3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
    let sentences = [
        ("first", "A song and a Christmas tree"),
        ("second", "Two candy canes"),
        ("third", "Three boughs of holly"),
        ("fourth", "Four colored lights"),
        ("fifth", "A shining star"),
        ("sixth", "Little silver bells"),
        ("seventh", "Candles a-glowing"),
        ("eighth", "Gold and silver tinsel"),
        ("ninth", "A guardian angel"),
        ("tenth", "Some mistletoe"),
        ("11th", "Gifts for one and all"),
        ("12th", "All their good wishes"),
    ];

    println!("Q3 : Sing 'The Twelve Days of Christmas' carol !\n");
    let mut carol = String::new();
    for sentence in sentences {
        carol = print_verse(sentence, carol);
        println!()
    }
}