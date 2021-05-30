fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", &v);

    let third = v.get(200);
    println!("third: {:?}", third);

    let v2 = vec![5, 6];
    for i in v2 {
        println!("{}", i);
    }
    let mut v3 = vec![5, 6, 200];
    for i in &mut v3 {
        *i += 50;
    }
    println!("{:?}", v3);
    let row = vec![
        SpreadsheetCell::Float(2.2),
        SpreadsheetCell::Int(114),
        SpreadsheetCell::Text(String::from("cuqui")),
    ];

    println!("{:?}", row);
    let mut s = String::new();
    s.push_str("quiqu");
    let data = "initial contents";
    s = data.to_string();
    println!("{:?}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let tic_tac_toe = s1 + &s2 + &s3;
    println!("{}", tic_tac_toe);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let tic_tac_toe = format!("{}-{}-{}", &s1, &s2, &s3);
    println!("{}", tic_tac_toe);
    let len = String::from("Здравствуйте").len();
    println!("{}", len);

    let chineese = String::from("杜鹃蜂蜜和土豆");
    // panics cause first byte is inside a 3 byte thing
    // this one: 杜
    // let s = &chineese[0..1];
    // println!("{}", s);
    let s = &chineese[0..3];
    println!("{}", s);
    // for c in chineese.chars() {
    for c in chineese.bytes() {
        println!("{}", c);
    }

    let mut map = HashMap::new();
    map.insert(String::from("blue"), 10);

    map.entry(String::from("cecil")).or_insert(10);
    println!("{:?}", map);
    let heroes = vec![String::from("coco"), String::from("cuco")];
    let power = vec![125, 167];
    let scores: HashMap<&String, &i32> = heroes.iter().zip(power.iter()).collect();

    println!("{:?}", scores);

    println!("Coco's score {:?}", scores.get(&String::from("coco")));

    for (key, value) in &scores {
        println!("{} is worth: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

use std::collections::HashMap;

// holding different types via enum
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
