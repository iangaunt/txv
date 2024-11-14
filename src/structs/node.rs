#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    pub val: i32,
    pub next: Option<Box<Node>>
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node { val, next: None }
    }
}