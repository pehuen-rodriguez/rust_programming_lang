pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules {
                println!("I was nested!");
            }
        }
    }
}

use a::series::of::nested_modules;

fn main() {
    nested_modules();
}
