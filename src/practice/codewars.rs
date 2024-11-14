fn ends_with(s: &str, end: &str) -> bool {
    if s.len() < end.len() { return false; }

    let mut s_chars = s.chars();
    let mut end_chars = end.chars();

    for _i in 0..end.len() {
        let end_ch: char = end_chars.next_back().unwrap();
        let s_ch: char = s_chars.next_back().unwrap();

        if end_ch != s_ch {
            return false;
        }
    }

    true
    
    // can also be done with s.ends_with(end)
}

fn split_string(s: &str) -> Vec<String> {
    let mut vec = Vec::new();
    let mut curr: String = String::from("");

    let mut i = 0;
    while i < s.len() {
        let ch: char = s.chars().nth(i).unwrap();
        curr.push(ch);

        if curr.len() == 2 {
            vec.push(curr);
            curr = String::from("");
        }

        i += 1;
    }

    if curr.len() == 1 {
        curr.push('_');
        vec.push(curr);
    }
    
    vec
}

fn mask(cc: &str) -> String {
    if cc.len() <= 4 { return String::from(cc); }

    let mut masked: String = String::from("");
    for i in 0..cc.len() {
        if i > cc.len() - 5 {
            masked.push(cc.chars().nth(i).unwrap());
        } else{
            masked.push('#');
        }
    }

    masked
}

fn short_mask(cc: &str) -> String {
    let mask_length = cc.len().saturating_sub(4);
    "#".repeat(mask_length) + &cc[mask_length..]
}

pub fn test_questions() {
    println!("{}", ends_with("abc", "bc"));
    println!("{:?}", split_string("abcde"));
    println!("{}, {}", mask("4556364607935616"), short_mask("4556364607935616"));
}