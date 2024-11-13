fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn char_assert() {
    let mut str: String = String::from("");

    for c in 'a'..='z' {
        let cn: u32 = c as u32;
        let mut k: String = cn.to_string();
        k.push_str(" ");
        
        str.push_str(k.as_str());
    }

    println!("{str}");
}

fn num_assert() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    println!("Success!");
}

fn return_void() {
    println!("I will return a ()");
}

fn copy_testing() {
    let x = (1, 2, (), "hello".to_string());
    let y = x;
    println!("{:?}, {:?}", y, y);
}

fn get_hello() -> String {
    let s = String::from("hello"); s
}

pub fn first_word(s: &mut String) -> String {
    let mut str: String = String::from("");

    for c in s.as_str().chars() {
        if c == ' ' {
            break;
        }
        str.push(c);
    }

    return str;
}

pub fn test() {
    get_hello();
    char_assert();
    num_assert();
    assert!(return_void() == ());
    copy_testing();
}