// use std::io;
// #[derive(Debug)]
// struct Data {
//     name: String,
//     marks: i32,
// }
// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Color(u32, u32, u32),
//     Write(String),
// }
#[derive(Debug)]
enum Coin {
    Penny,
    Dime,
    Nickel,
    Quarter,
}

fn which_coin(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("This is a penny loser");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
    // let mut name = String::new();
    // io::stdin().read_line(&mut name).expect("err");
    // println!("Welcome {}", name);
    // println!("Add names and their marks with space in between");
    // let mut name_marks: Vec<Data> = Vec::new();
    // for _ in 0..5 {
    //     let mut incoming = String::new();
    //     io::stdin().read_line(&mut incoming).expect("some");
    //     let mut iter = incoming.split_whitespace();
    //     let name = iter.next().expect("Missing name").to_string();
    //     let marks = iter
    //         .next()
    //         .expect("Missing marks")
    //         .parse()
    //         .expect("Invalid marks");
    //     let d = Data { name, marks };
    //     name_marks.push(d);
    // }
    // name_marks.sort_by(|a, b| b.marks.cmp(&a.marks));
    // for data in name_marks.iter() {
    //     println!("name is {} and marks is {}", data.name, data.marks);
    // }
    // println!("{:?}", [1, 2, 3].map(|f| i32::pow(f, 2)))
    // let ipv4 = IpAddr::V4(String::from("127.0.0.1"));
    // let ipv6 = IpAddr::V6(String::from("::1"));
    // println!("{:?}", ipv4);
    // println!("{:?}", ipv6);
    // let some_num = Some(4);
    // let some_char = Some(b'C');
    // let none_num: Option<i32> = None;
    // let non_option: i32 = 45;
    // println!("{}", non_option + none_num);
    let c = Coin::Dime;
    println!("{}", which_coin(&c));
    let penny = Coin::Penny;
    let ret = which_coin(&penny);
    println!("{:?}", ret);
    println!("{:?}", c);
}
