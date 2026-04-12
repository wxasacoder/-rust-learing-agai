use std::io::{self, Write};

/// 解析并计算一个简单表达式 "a op b" (支持 + - * /)
fn calculate(a: f64, op: &str, b: f64) -> Option<f64> {
    match op {
        "+" => Some(a + b),
        "-" => Some(a - b),
        "*" => Some(a * b),
        "/" => {
            if b == 0.0 {
                None
            } else {
                Some(a / b)
            }
        }
        _ => None,
    }
}

fn main() {
    println!("=== 简易计算器 ===");
    println!("支持运算: + - * /");
    println!("输入格式: 数字 运算符 数字 (例如: 3.14 + 2.86)");
    println!("输入 'q' 退出\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("q") {
            println!("再见!");
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            println!("错误: 请输入 '数字 运算符 数字' 格式");
            continue;
        }

        let a: f64 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("错误: '{}' 不是有效的数字", parts[0]);
                continue;
            }
        };

        let op = parts[1];
        let b: f64 = match parts[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("错误: '{}' 不是有效的数字", parts[2]);
                continue;
            }
        };

        match calculate(a, op, b) {
            Some(result) => println!("{} {} {} = {}", a, op, b, result),
            None => println!("错误: 运算符 '{}' 不支持或除以零", op),
        }
    }
}
