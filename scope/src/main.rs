fn main() {
    {
        let s = "string";
        println!("String here was {}", s);
    }
    const ES: &str = "string";
    println!("String isn't valid ouside that scope. {}", ES);

    let mut another_string = String::from("like this one");

    another_string.push_str(" con algo más");
    println!("String type is allocated on the heap. {}", another_string);
    let this_string = another_string;
    println!("They both point to the same memory space. {}", this_string);
}
