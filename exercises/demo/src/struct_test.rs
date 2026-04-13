use rust_decimal::Decimal;
use std::fmt::{Display, Formatter};
use std::ops::Mul;

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
    println!("元祖:{}", u);

    let person_one = UserId(18);
    let person_two = UserId(23);
    println!("{}", person_one.get_age_minus());
    println!("{}", person_two.get_age_minus());

    println!("one 是否被 two 年轻{}", person_one.is_younger(&person_one));

    let price  = Decimal::new(10,2);
    let total_count =  Decimal::new(23,0);
    person_two.pay(&UserId::calculate(&price, &total_count))

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

impl UserId {
    fn get_age_minus(&self) -> u32 {
        self.0 * 60
    }
}

impl UserId{
    fn is_younger(&self, other: &UserId)-> bool{
        self.0 < other.0
    }
}



impl Pay for UserId{
    fn pay(&self, total_amount: &Decimal) -> () {
        println!("支付了{}", total_amount);
    }

    fn calculate(price: &Decimal, count: &Decimal) -> Decimal {
        price.mul(count)
    }
}


trait Pay{
    fn pay(&self, total_amount: &Decimal) ->();
    fn calculate(price: &Decimal, count: &Decimal) -> Decimal;
}

