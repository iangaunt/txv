mod node { include!("../structs/node.rs"); }

use node::Node;

pub struct Rectangle {
    width: i32,
    height: i32
}

// Implementation of a Rectangle struct.
impl Rectangle {
    // Constructors.
    fn new(width: i32, height: i32) -> Self { Self { width, height } }
    fn square(size: i32) -> Self { Self { width: size, height: size }}

    fn area(&self) -> i32 { self.width * self.height }
    fn width(&self) -> bool { self.width > 0 }

    fn can_fit(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[derive(Debug)]
struct Circle {
    radius: i32
}

pub struct Account {
    username: String,
    password: String,
    number: i32,
    balance: f64
}

// Tuple structs.
#[derive(Debug)]
struct Color(i32, i32, i32);

pub fn print_account(a: &Account) {
    println!("{} ({}): {}, {}", a.username, a.password, a.number, a.balance);
}

pub fn make_shapes() {
    let mut rect: Rectangle = Rectangle::new(10, 20);
    let rect2: Rectangle = Rectangle::new(5, 7);
    let square: Rectangle = Rectangle::square(11);

    if rect.width() { 
        println!("The area of the rectangle is {} units", rect.area());
    }
    println!("Can rect2 fit in rect? {}", rect.can_fit(&rect2));
    println!("Can square fit in rect? {}", rect.can_fit(&square));

    let mut circ: Circle = Circle {
        radius: 10
    };
    println!("{circ:?}");

    let mut acc: Account = Account {
        username: String::from("john"),
        password: String::from("password123"),
        number: 12345,
        balance: 1000.0
    };

    let acc2: Account = Account {
        number: 29384,
        password: String::from("secret"),
        ..acc
    };

    rect.width = 15;
    rect.height = 20;
    circ.radius = 15;
    acc.username = String::from("bill");
    
    print_account(&acc);
    print_account(&acc2);

    let n1: Node = Node::new(10);
    let mut n2: Node = Node::new(20);
    n2.next = Some(Box::new(n1));

    let k: Color = Color(10, 20, 30);
    println!("{:?}, {:?}, {:?}", k.0, k.1, k.2);
}