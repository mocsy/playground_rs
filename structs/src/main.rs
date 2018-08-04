fn main() {
    let mut user0 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    //String moves, u64,bool does not
    let user1 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user0
    };
    println!("0 username {} 1 username {}", user0.username, user1.username);
    user0.username = String::from("watanuki");
    println!("0 username {} 1 username {}", user0.username, user1.username);
    println!("0 active {} 1 active {}", user0.active, user1.active);
    user0.active = false;
    println!("0 active {} 1 active {}", user0.active, user1.active);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.",
        rect1.area()
    );
        println!("The area*10 of the rectangle is {} square pixels.",
        rect1.area10()
    );
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
//Tuple sturcts
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
//Unit-Like Struct
struct UnitLike {}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    fn area10(&self) -> u32 {
        self.width * self.height * 10
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
