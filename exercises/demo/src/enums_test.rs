use std::fmt::{Display, Formatter};

fn main() {
    let  a = IpAddr::V4(1,2,4,5);
    println!("a is:{:?}", a );
}
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),   // 每个变体可携带不同类型的数据
    V6(String),
}


enum Option<T> {
    Some(T),
    None,
}