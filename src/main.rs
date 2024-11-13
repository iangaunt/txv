mod testing;
mod tutorial { include!("tut/tutorial.rs"); }

fn main() {
    let play_guessing: bool = false;
    let testing: bool = false;
    let tutorial_use: bool = false;

    if tutorial_use {
        tutorial::looper();
        tutorial::variables();
        tutorial::logic_practice();
        tutorial::ownership();
        tutorial::pointers();
    }
    if play_guessing { tutorial::guessing(); }
    if testing { testing::test(); }

    let mut str: String = String::from("goodbye world");
    let s_str: &mut String = &mut str;

    let first_word: String = testing::first_word(s_str);
    println!("{first_word}");
}
