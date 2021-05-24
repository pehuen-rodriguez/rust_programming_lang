fn main() {
    {
        let s = "string";
        println!("String is only valid within the scope: {}", s);
    }
    const ES: &str = "string";
    println!("Strings as pointers: {}", ES);

    let mut another_string = String::from("like this one");

    another_string.push_str(" con algo más");
    println!("String type is allocated on the heap: {}", another_string);
    let this_string = another_string;
    println!("They both point to the same memory space: {}", this_string);
    // Can't use another_string anymore because its been moved
    // thus preventing double memory free
    // println!("String type is allocated on the heap. {}", another_string);
    
    // But it can be cloned in the heap
    another_string = this_string.clone();
    println!("Clone in the heap: {}", another_string);
    
    let x = 5;
    let y = x;
    println!("Copies are valid on the stack: {} and {}", x, y);
}
