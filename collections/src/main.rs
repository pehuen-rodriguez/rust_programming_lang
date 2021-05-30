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
}

// holding different types via enum
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
