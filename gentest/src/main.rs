// #![feature(never_type)]

trait Me {
    fn print_me<T>(me: T) where Self: Sized;    
}
impl dyn Me {
    fn print_me_too<T: std::fmt::Display + std::fmt::Debug>(me: T) {
        println!("{:?}", me);
    }
    fn print_me<T: std::fmt::Display + std::fmt::Debug>(me: T) {
        println!("{:?}", me);
    }
}

struct Z();
impl Me for Z {
    fn print_me<T> (_me: T) {
        panic!("waaa");
    }
}

fn main() {
    // works
    <dyn Me>::print_me_too::<String>("aa".to_owned());

    // error[E0034]: multiple applicable items in scope
    // note: candidate #1 is defined in an impl for the type `(dyn Me + 'static)`
    // Shouldn't <dyn Me + 'static> disambiguate it?
    // <dyn Me + 'static>::print_me::<String>("aa".to_owned());
    
    // works
    <Z as Me>::print_me::<String>("aa".to_owned());
}
