
fn main() {
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("{:#?}", third);
    let third: Option<&i32> = v.get(2);
    println!("{:#?}", third);
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    println!("{:#?}", does_not_exist);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("{:#?}, {}", v, first);
    //v.push(6); //v borrowed already

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:#?}", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:#?}", row);
}