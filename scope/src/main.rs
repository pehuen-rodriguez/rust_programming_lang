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
    println!(
        "Fixed length ints are cheap to copy in the stack: {} and {}",
        x, y
    );

    let this_string = String::from("this string");
    let this_string = returns_ownership_back(this_string);
    takes_ownsership(this_string); // this_string value moved
    let an_int: i64 = 123;
    makes_copy(an_int); // ints values are copied
    println!("Still valid here: {}", an_int);
    // can also get ownership from function
    let received = gives_ownsership();
    println!("I got a value: {}", received);
    let s = String::from("hello");
    let (s, length) = calculate_length(s);
    println!("Length was calculated: {}", length);
    println!("And s was back: {}", s);
    let length = calculate_length_borrow(&s);
    println!("Length was calculated: {}", length);
    println!("And s wasn't gone: {}", s);

    let mut can_be_muted = String::from("I'll be muted");
    mutates_a_reference(&mut can_be_muted);
    println!("And s was mutated: {}", can_be_muted);
    
    let like_this = &mut can_be_muted;
    // second mutable borrows forbidden
    // to prevent data races
    // let but_not_like_this = &mut can_be_muted;
    mutates_a_reference(like_this);
    println!("And was also mutated: {}", like_this);
    // mutates_a_reference(but_not_like_this);
    
    let _unmutable_reference = &can_be_muted;
    // cant be borrowed as mutable after inmutable
    // inmutable couldn't expect its value to change
    // let mutable_reference = &mut can_be_muted;
    // println!("And was also mutated: {}", mutable_reference);

    // let cant_make_dangling_ref = dangle();
}

// cant return dangling reference
// "will_drop" will be dropped and no value
// will exists for for a reference to
// fn dangle() -> &String {
//     let will_drop = String::from("No");
//     &will_drop
// }

fn gives_ownsership() -> String {
    println!("I'm only giving the string back");
    String::from("I'm a value")
}

fn takes_ownsership(s: String) {
    println!("Now this function owns: {}", s);
}

fn makes_copy(i: i64) {
    println!("This didn't take ownership: {}", i);
}

fn returns_ownership_back(s: String) -> String {
    println!("I'll print it and give it back: {}", s);
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_borrow(s: &String) -> usize {
    s.len()
}

fn mutates_a_reference(s: &mut String) {
    s.push_str(" con algo más");
}
