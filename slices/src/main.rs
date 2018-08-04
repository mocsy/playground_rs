fn main() {
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("a: {} b: {}", hello, world);

    let f = first_word(&s);
    println!("first word: {}", f);
    
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];    
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
