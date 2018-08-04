
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    // addition
    let sum = 5 + 10;

    println!("x : {} y : {} sum : {}", x,y,sum);

    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let modulo = 43 % 5;
    println!("difference : {} product : {} quotient : {} modulo : {}",
     difference,product,quotient,modulo);

    let c = 'z';
    let z = 'ℤ';
    let cat = '😻';
    println!("c : {} z : {} cat : {}", c,z,cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x : {} y : {} z : {}", x,y,z);
    println!("1 : {} 2 : {} 3 : {}", tup.0, tup.1, tup.2);

    let a = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let first = a[0];
    let second = a[1];

    println!("first : {} second : {}", first, second);
    // index out ot bounds
    // let index = 10;
    // let element = a[index];
    // println!("The value of element is: {}", element);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("x : {} y : {}", x,y);

    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(5);
    println!("The value of x is: {}", x);

    if x < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    if x != 0 {
        println!("x was something other than zero");
    }

    if x % 4 == 0 {
        println!("x is divisible by 4");
    } else if x % 3 == 0 {
        println!("x is divisible by 3");
    } else if x % 2 == 0 {
        println!("x is divisible by 2");
    } else {
        println!("x is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
