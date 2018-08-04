
#[derive(Debug)]
enum Goblins {
    Cons(i32, Box<Goblins>),
    Nil,
}

use Goblins::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1,
                Box::new(Cons(2,
                    Box::new(Cons(3,
                    Box::new(Nil))))));
    println!("{:?}", list);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
