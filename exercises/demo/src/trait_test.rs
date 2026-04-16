fn main() {

}



pub trait Display {
    fn show_color(&self, c: &str) -> ();
}

impl show_color for Display  {
    fn show_color(&self, c : &str) -> () {
        println!("现在的颜色是:{}", c);
    }
}