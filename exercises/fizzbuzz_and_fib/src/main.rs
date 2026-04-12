use std::io::{self, Write};

/// FizzBuzz: 输出 1 到 n 的数字, 但 3 的倍数输出 "Fizz", 5 的倍数输出 "Buzz",
/// 同时是 3 和 5 的倍数输出 "FizzBuzz"
fn fizzbuzz(n: u32) {
    for i in 1..=n {
        match (i % 3, i % 5) {
            (0, 0) => print!("FizzBuzz "),
            (0, _) => print!("Fizz "),
            (_, 0) => print!("Buzz "),
            _ => print!("{} ", i),
        }
    }
    println!();
}

/// 斐波那契数列: 使用迭代法, 返回第 n 个斐波那契数
fn fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn main() {
    println!("=== FizzBuzz (1 到 30) ===");
    fizzbuzz(30);

    println!("\n=== 斐波那契数列 (前 20 项) ===");
    for i in 0..20 {
        print!("{} ", fibonacci(i));
    }
    println!();

    println!("\n=== 自定义输入 ===");
    print!("请输入一个数字运行 FizzBuzz (输入 q 退出): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "q" {
        println!("再见!");
        return;
    }

    if let Ok(n) = input.parse::<u32>() {
        println!("\nFizzBuzz (1 到 {}):", n);
        fizzbuzz(n);

        println!("\n斐波那契前 {} 项:", n.min(30));
        for i in 0..n.min(30) {
            print!("{} ", fibonacci(i));
        }
        println!();
    } else {
        println!("无效的输入: {}", input);
    }
}
