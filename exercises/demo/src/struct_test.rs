use std::fmt::{Display, Formatter};

fn main(){

    let user = User{
        username: String::from("bob"),
        age: 1
    };

    println!("Username: {}", user);

    let s = Rectangle::square(10);
    let  s_area = s.area();
    println!("正方行的大小{}", s_area);

    let u =  UserId(1);
    println!("元祖:{}", u)
}

struct User<>{
    username: String,
    age: i32,
}
// 实现display
impl<> Display for User<> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "User({}, {})", self.username, self.age);
    }
}

struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle {

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self{width: size, height: size}
    }


}

struct UserId(u32);

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "UserId({})", self.0);
    }
}