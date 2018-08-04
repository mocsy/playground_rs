#[cfg(test)]
mod tests {
    use super::client;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        client::connect();
    }
}

pub mod client;
pub mod network;
