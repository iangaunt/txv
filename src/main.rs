use rand::Rng;
use std::cmp::{min, Ordering};
use std::io;

/**
  Plays a guessing game with the user, selecting a random i32 from 0 to 100
  and prompting the user to select a value in the terminal until they are correct. 
 */
fn guessing() {
    println!("Guessing game!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");
    
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}!");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn sum(a: i32, b: i32) -> i32 { a + b }
fn square(a: i32) -> i32 { a * a }

/**
  Performs various variable calculations and outputs them for the user.
 */
fn variables() {
    const THREE: u32 = 3;

    // Defining new variables.
    let mut x = 5;
    println!("The value of x is {x}");

    // Changing the value of a "mut" (mutable) variable.
    x = THREE;
    println!("The value of x is {x} now");

    // A codeblock nestled inside a function with no return.
    {
        let x = 7;
        println!("The value of x is {x} now");
    }
    println!("The value of x is {x} now");

    // Defining floats.
    const PI: f32 = 3.1415926;
    let mut _r = 2.5; // defaults to f64
    _r *= PI;

    // Characters.
    let _c = 'z';

    // Tuples with multiple types.
    let _tup: (i32, char) = (100, 'z');

    // Collections / arrays with a datatype and length.
    let _a: [i32; 3] = [1, 3, 5];
    let _b = [3; 5];

    // Function calls.
    let mut k = sum(10, 20);
    println!("The value of k is {k}");

    k = square(10);
    println!("The value of k is now {k}");

    // Variables can be defined with lamba expressions.
    let mut y = {
        let x = 5;
        (x + 1) * 2
    };
    y = y * 2;

    println!("The value of y is {y}");
}

/**
  Performs the fizzbuzz game on an integer value.
 */
fn fizz_buzz(n: i32) {
    // If-else if-else logic.
    if n % 3 == 0 && n % 5 == 0 {
        println!("FizzBuzz");
    } else if n % 3 == 0 {
        println!("Fizz");
    } else if n % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{n}");
    }
}

/**
  Function for performing various loops.
 */
fn looper() -> i32 {
    let mut counter = 0;

    // Keeps looping until the counter variable is 10.
    let result = loop {
        counter += 1;
        fizz_buzz(counter);

        if counter == 100 {
            // Break can return a value.
            break counter;
        }
    };

    // Internal loops. Loops with no return must start with '.
    counter = 10;
    'count: loop {
        'internal: loop {
            if counter == 10 {
                break 'internal;
            }
        }

        break 'count;
    };

    // While loops.
    let mut rocket = 10;
    while rocket > 0 {
        println!("{rocket}");
        rocket -= 1;
    }
    println!("LIFTOFF!!");

    // Looping through a collection.
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("{element}");
    }

    // Looping through a range.  
    // Prints 1 through 9. (inclusive .. non inclusive)
    for i in 1..10 {
        println!("{i}");
    }

    // Prints 1 through 10. (inclusive .. =inclusive)
    for i in 1..=10 {
        println!("{i}");
    }

    return result;
}

fn f_to_c(f: f32) -> f32 { (f - 32.0) * (5.0 / 9.0) }
fn c_to_f(f: f32) -> f32 { (f * 1.8) + 32.0 }

/**
  Calculates the zero-indexed nth Fibonacci number.
 */
fn nth_fib(mut i: i64) -> i64 {
    if i < 2 { return i; }

    let mut one = 0;
    let mut two = 1;

    while i > 0 {
        if one < two {
            one = one + two;
        } else {
            two = one + two;
        }

        i -= 1;
    }

    return min(one, two);
}

fn main() {
    looper();
    variables();

    let fc: f32 = f_to_c(c_to_f(32.0));
    println!("{fc}");

    let fib: i64 = nth_fib(4);
    println!("{fib}");
    
    // guessing();
}