fn main() {

}

#[test]
fn test_r_some(){
    let  color_show  = ColorShow::new();
    test_t_som(&color_show);
    test_dyn(&color_show);
    let breaking_bak_tv = BreakingBad::new();
    breaking_bak_tv.show_tv("1");
    breaking_bak_tv.show_color("蓝绿色");

    let b: &dyn TvShow =  &breaking_bak_tv;
    b.show_product("auxin")

}



pub trait Display {
    fn show_color(&self, c: &str) -> ();
}


fn test_dyn(d: &dyn Display) -> (){

    d.show_color("black");

}
fn test_t_som<T: Display> (t: &T) -> (){
    t.show_color("red");
}


struct ColorShow {}

impl Display for ColorShow  {
    fn show_color(&self, c : &str) -> () {
        println!("现在的颜色是:{}", c);
    }
}


pub trait TvShow{
    fn show_tv(&self,s: &str) where Self: Sized;
}


pub trait ProductPrinter{
    fn show_product( &self, s: &str);
}

impl <T: TvShow> Display for T {
    fn show_color(&self, c: &str) -> () {
        println!("电视的主题色是{}", c);
    }


}


impl ProductPrinter for dyn TvShow {
    fn show_product(&self, s: &str)
    {
        println!("制作人{}", s);
    }
}




impl ColorShow {
    fn new() -> Self {
        Self{

        }
    }
}

struct BreakingBad{


}


impl BreakingBad{
    fn new() -> Self {
        Self{}
    }
}


impl TvShow for BreakingBad{
    fn show_tv(&self,s: &str)
    where
        Self: Sized
    {
        println!(" 现在播放绝命毒师第{}集", s);
    }
}
