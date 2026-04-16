use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::Color::Red;

fn main() {
    let  a = IpAddr::V4(1,2,4,5);
    println!("a is:{:?}", a );
    let c = Color::Blue(1);
    let d = Color::Red("read".to_string());
    let e: Option<u32>  = Some(5);
    println!("{}", e.expect("error"));
    println!("{}", e.unwrap_or(0));
    // let f: Option<u32> = None;
    // let eu = f.expect("exception----------->");
    // println!("{}", eu);
}

#[test]
fn  test_value_color(){
    let a  = value_in_color(Color::Blue(1));
    println!("{:?}", a);
}


#[test]
fn let_ele_test (){
   let a   = Some(1);
    if let Some(b) = a {
        println!("{}", b);
    }

}

fn value_in_color(need_match: Color) -> Color{
    match need_match {
        Red(_) => Red("aa".to_string()),
        other => other
    }
}

#[derive(Debug)]
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