use std::io;
pub fn change2(s: &mut String) {
    *s = String::from("hell");
}
pub fn change(mut _s: String) -> String {
    // _s = String::from("some new");
    _s.push_str(" world");
    _s
}
pub fn first_word(s: String) -> String {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return s[0..i].to_string();
        }
    }
    return s[..].to_string();
}
fn main() {
    // let mut s = String::from("hello");
    // s.push_str(" world");
    // println!("{}", s);
    // let mut s = String::from("hell");
    // s.push_str(" world");
    // let mut s = String::new();
    // io::stdin().read_line(&mut s).expect("err");
    // let s = change(s);
    // println!("{}", s);
    // let mut s = String::new();
    // io::stdin().read_line(&mut s).expect("err");
    // change2(&mut s);
    // println!("{}", s);
    // let slice = &s[1..4];
    // println!("{}", slice);
    // let s = String::from("hello world");
    // let word = first_word(s);
    // println!("{}", s);
    // println!("{}", word);
    // let a = [1, 2, 3, 4, 5];
    // let slice = &a[..3];
    // println!("{:?}", slice);
}
