extern crate communicator;

fn main() {
    communicator::client::connect();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        client::connect();
    }
}
