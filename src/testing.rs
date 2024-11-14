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

pub fn first_word(s: &str) -> &str {
    for i in 0..s.len() {
        let c: char = s.chars().nth(i).unwrap();

        if c == ' ' { 
            return &s[0..i]; 
        }
    }

    return s;
}

pub fn second_word(s: &str) -> &str {
    let mut space_found: bool = false;
    let mut space_index: usize = 0;

    for i in 0..s.len() {
        let c: char = s.chars().nth(i).unwrap();
        if c == ' ' {
            if !space_found { 
                space_found = true; 
                space_index = i;

                continue; 
            } else {
                return &s[space_index + 1..i];
            }
        }
    }

    if space_found { return &s[space_index + 1..]; }
    return s;
}

pub fn pointer_stuff() {
    let mut str: String = String::from("testing");

    // You can have multiple mutable pointers, just not at the same time.
    {
        let sp: &mut String = &mut str;
        sp.push_str(" p1")
    }

    let r: &mut String = &mut str;
    r.push_str(" p2");

    {
        let s: &String = &str;
        println!("{} {}", &s, str);
    }

    // Two different approaches to counting pointer values.
    let mut nums: String = String::from("");
    let nums_p: &mut String = &mut nums;

    for i in 1..=10 {
        let st: String = i.to_string() + " ";
        nums_p.push_str(st.as_str());
    }

    println!("{nums}");

    nums = String::from("");
    for i in 1..=10 {
        let st: String = i.to_string() + " ";
        nums.push_str(st.as_str());
    }

    {
        let x: i32 = 5;
        let p: &i32 = &x;
        println!("{:p}", p);
    }
}

pub fn test() {
    let run: bool = false;

    if run {
        get_hello();
        char_assert();
        num_assert();
        assert!(return_void() == ());
        copy_testing();
        
        pointer_stuff();

        let str: String = String::from("goodbye world");
        let first_word: &str = first_word(&str);
        let second_word: &str = second_word(&str);
    
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let k: &[i32] = &a[0..2];
        
        assert_eq!(k, &[1, 2]);
    
        println!("{}, {}", first_word, second_word);
    }
}