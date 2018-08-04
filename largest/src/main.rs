
fn largest<T>(list: &[T]) -> T
where T: PartialOrd + Copy {
    let l = list.len();
    if l == 1 {
        list[0]
    } else {
        let c = list[0];
        let n = largest(&list[1 .. ]);
        if c < n {n} else {c}
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
