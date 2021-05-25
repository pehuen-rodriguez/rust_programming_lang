pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules {
                println!("I was nested!");
            }
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}
