use std::io;
fn add(x:i32,y:i32) -> i32{
 return x+y
}

fn main() {
    // let mut x = 4;
    // println!("{x}");
    // x = 42;
    // println!("{x}");
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("{}", THREE_HOURS_IN_SECONDS);
    // let spaces = "            ";
    // println!("{}", spaces);
    // let spaces = spaces.len();
    // println!("{}", spaces);
    // let i: i32 = -123;
    // println!("{i}")
    // let char: u8 = b'A';
    // println!("{char}");

    // let a: i32 = 2;
    // let b: i32 = i32::MAX;
    // println!("{}", a.wrapping_add(b));
    // let c = 'z';
    // let z: char = 'ℤ';
    // let heart_eyed_cat = '😻';
    // println!("{}", c);
    // println!("{}", z);
    // println!("{}", heart_eyed_cat);
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("{}", y);
    // println!("{}", z);
    // println!("{}", tup.1);
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // // println!("{}", a[0]);
    // // let zeroes = [0; 6];
    // let mut string: String = String::new();
    // io::stdin()
    //     .read_line(&mut string)
    //     .expect("Error");
    // let string: usize = string
    //     .trim()
    //     .parse()
    //     .expect("Error in parsing");
   // println!("{}", a[string]);
    // println!("{}",add(2,2));
   // for i in (1..5).rev(){
   //     println!("{}",i);
   // } 
    // Program to convert farenheit to celsius and other way round
    let mut x :String = String::new();

    io::stdin()
        .read_line(&mut x)
        .expect("err");


    let mut x: i32 = x.trim().parse().expect("err");
    x = (x-32)*5/9;
    println!("{}",x);
}

