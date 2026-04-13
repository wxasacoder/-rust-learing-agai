fn main() {
    let left: i32 = 1;
    let right: i32 = 2;
    let str: String = String::from("hello");
    pt_str(str);
    let i = calculate_add(left, right);
    let  a:&str = if i % 2 == 0 {
        "奇数"
    } else {
        "偶数"
    };
    println!("{}", left);
    println!("{}", right);
    println!("{}", a);
}

fn calculate_add(a: i32, b: i32) -> i32 {
    a + b
}

fn pt_str(a :String){
    println!("{}", a);
}
