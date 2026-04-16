use std::ptr::slice_from_raw_parts;

#[test]
fn test_vector()  {
    let mut a:Vec<&str>  = Vec::new();
    let b:Vec<i32>   = vec![1,2,3];

    a.push("hello");


    let  h = a[0];
    let  d = a[0];
    println!("{}", d);
    a.push("world");
    let  w = a[1];
    println!("{}{}", h, w);

    for i in &a{
        println!("{}", i);
    };

    for e in b {
        println!("{}", e);
    }

    let mut str  = String::new();
    let str_2= "world".to_string();
    str.push_str("hello");
    str.push_str(" world");
    println!("{}", str);

    let str_4  = str + &str_2;
    println!("{}{}", str_4, str_2);


    let s4 = format!("{str_2}-{str_4}");      // 推荐，不移动所有权
    println!("{}", s4);
}


fn main() {


}