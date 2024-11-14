use rand::Rng;
use std::cmp::{min, Ordering};
use std::io;

/**
 Plays a guessing game with the user, selecting a random i32 from 0 to 100
 and prompting the user to select a value in the terminal until they are correct.
*/
pub fn guessing() {
    println!("Guessing game!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

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

pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn square(a: i32) -> i32 {
    a * a
}

/**
 Performs various variable calculations and outputs them for the user.
*/
pub fn variables() {
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
pub fn fizz_buzz(n: i32) {
    let mut str = String::from("");

    if n % 3 == 0 {
        str.push_str("Fizz");
    }
    if n % 5 == 0 {
        str.push_str("Buzz");
    }

    if str.len() == 0 {
        str = n.to_string();
    }

    println!("{str}");
}

/**
 Function for performing various loops.
*/
pub fn looper() -> i32 {
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
    }

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

pub fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * (5.0 / 9.0)
}
pub fn c_to_f(f: f32) -> f32 {
    (f * 1.8) + 32.0
}

/**
 Calculates the zero-indexed nth Fibonacci number.
*/
pub fn nth_fib(mut i: i64) -> i64 {
    if i < 2 {
        return i;
    }

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

pub fn logic_practice() {
    let fc: f32 = f_to_c(c_to_f(32.0));
    println!("{fc}");

    let fib: i64 = nth_fib(4);
    println!("{fib}");
}

pub fn ownership() {
    // String literal; cannot be mutated.
    /*
        let mut str2 = "Hello!";
        str2 += ", world!"; <-- throws an error
    */

    // The string str is dropped by Rust when it goes out of scope.
    {
        let mut str: String = String::from("Hello");
        println!("{str}");

        str.push_str(", world!");
        println!("{str}");
    }

    // Primitive types will point to different locations, as they are of fixed size.
    let y = 5;
    let x = y;
    sum(x, y);

    {
        let str1: String = String::from("owner");
        let str2 = str1; // Points to the same location on the heap as str1.

        // When these both go out of scope, they will attempt to double-free the memory.

        // str1 can no longer be used after str1 is created, because the value of str1 has
        // been MOVED to str2.

        // println!("{str1}"); <- "borrow of moved value" error
        println!("{str2}"); // OK
    }

    // What if you don't want to move the data?
    {
        let str1: String = String::from("deepcopy");
        let str2 = str1.clone(); // DEEP COPIES the data stored in str1, data is not moved.

        println!("{str1}"); // OK
        println!("{str2}"); // OK
    }

    // Assigning a value to another variable will move it.
}

pub fn str_size(s: &String) -> usize {
    return s.len();
}

pub fn change_str(s: &mut String) {
    s.push_str(" modified");
}

pub fn pointers() {
    let mut str: String = String::from("testing");
    let sp: &mut String = &mut str;

    // When s goes out of scope, because this scope does not own the string
    // value passed in, it will not be dropped.

    let str_s = str_size(sp); // Pass in a pointer to save ownership.
    println!("{str_s}"); // OK

    change_str(sp);
    sp.push_str("!"); // OK, mutable reference

    println!("{str}");

    // There can only be one mutable reference to a variable at a time.
    // known as a "data race"

    // You cannot have a mutable reference to a variable and an immutable one at the same time.
    // You CAN have multiple IMMUTABLE references.

    // Rust will not compile if you have dangling pointers (pointers to variables which are dropped)
}

pub fn tutorial() {
    let play_guessing: bool = false;
    let tutorial_use: bool = false;

    if tutorial_use {
        looper();
        variables();
        logic_practice();
        ownership();
        pointers();
    }
    
    if play_guessing { guessing(); }
}