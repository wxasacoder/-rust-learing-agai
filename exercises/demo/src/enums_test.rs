use std::fmt::{Display, Formatter};

fn main() {
    let  a = IpAddr::V4(1,2,4,5);
    println!("a is:{:?}", a );
    let c = Color::Blue(1);
    let d = Color::Red("read".to_string());
    let e: Option<u32>  = Some(5);
    println!("{}", e.expect("error"));
    println!("{}", e.unwrap_or(0));
    let f: Option<u32> = None;
    let eu = f.expect("exception----------->");
    println!("{}", eu);
}


enum Color {
    Red(String),
    Green(u8),
    Blue(u8),
}



#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),   // 每个变体可携带不同类型的数据
    V6(String),
}