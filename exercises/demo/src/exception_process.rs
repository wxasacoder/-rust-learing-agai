use std::fs::File;
use std::io::Read;

fn main() {
    let content = read_file_to_string("./Cargo.toml");
    println!("{}", content);
}

#[test]
fn test_panic(){
    panic!("error occurred!!!");
}

#[test]
fn test_file_open(){
    let mut file = match File::open("./Cargo.toml") {
        Ok(file) => file,
        Err(e) => panic!("打开文件失败: {e:?}")
    };

    let mut content = String::new();
    file.read_to_string(&mut content).expect("读取文件内容失败");
    println!("{}", content);
}

/// 读取文件全部内容并返回 String
fn read_file_to_string(path: &str) -> String {
    let mut file = File::open(path).expect("打开文件失败");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("读取文件内容失败");
    content
}


