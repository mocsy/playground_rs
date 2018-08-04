extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("{}", s);
    let s = String::from("initial contents");
    println!("{}", s);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s2 is {}", s2);
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used
    println!("{}", s3);
   
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1.clone() + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let hola = String::from("Hola");
    println!("{} {}", hola, hola.len());
    let hello = String::from("Здравствуйте");
    println!("{} {}", hello, hello.len());
    let s = &hello[0..4];
    println!("{}", s);
    //let s = &hello[0..1]; //'byte index 1 is not a char boundary; it is inside 'З'

    let hello_hindi = "नमस्ते";
    for c in hello_hindi.chars() {
        println!("{}", c);
    }
    for b in hello_hindi.bytes() {
        println!("{}", b);
    }
    let g = UnicodeSegmentation::graphemes(&hello[..], true)
        .collect::<Vec<&str>>();
    println!("{:?}", g);
    let g = UnicodeSegmentation::graphemes(&hello_hindi[..], true)
        .collect::<Vec<&str>>();
    println!("{:?}", g);
}
