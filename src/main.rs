mod testing;
mod tutorial { include!("tut/tutorial.rs"); }
mod structs { include!("tut/structs.rs");}
mod practice { include!("practice/codewars.rs"); }

fn main() {
    testing::test();
    tutorial::tutorial();
    structs::make_shapes();
    
    practice::test_questions();
}
