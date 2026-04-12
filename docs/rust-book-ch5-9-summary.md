# Rust 速通笔记 · 第 5-9 章

> 涵盖：结构体、枚举、模块、集合、错误处理

---

## 第五章：结构体

### 5.1 定义与实例化

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

let user1 = User {
    active: true,
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    sign_in_count: 1,
};
```

**字段初始化简写**（参数名和字段名相同时）：
```rust
fn build_user(email: String, username: String) -> User {
    User { active: true, username, email, sign_in_count: 1 }
}
```

**结构体更新语法**：
```rust
let user2 = User {
    email: String::from("bob@example.com"),
    ..user1   // 其余字段从 user1 拷贝/移动
};
```

> `..user1` 必须在最后。如果剩余字段是 `String`，所有权会移走，`user1` 之后不可用。

### 5.2 元组结构体 & 单元结构体

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);  // 同是 3 个 i32，但不同类型！
let black = Color(0, 0, 0);
let x = black.0;              // 索引访问

struct AlwaysEqual;           // 无字段
let ae = AlwaysEqual;         // 用于实现不携带数据的 Trait
```

### 5.3 方法

```rust
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }

impl Rectangle {
    fn area(&self) -> u32 {           // &self = 不可变借用
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {    // 关联函数（构造函数）
        Self { width: size, height: size }
    }
}

let rect = Rectangle { width: 30, height: 50 };
println!("面积 = {}", rect.area());   // 80
let sq = Rectangle::square(10);       // :: 调用关联函数
```

**`dbg!` 宏** —— 调试利器（输出文件名、行号、值，返回所有权）：
```rust
let scale = 2;
let rect = Rectangle {
    width: dbg!(30 * scale),   // 打印: [src/main.rs:X] 30 * scale = 60
    height: 50,
};
dbg!(&rect);                    // 打印整个结构体
```

> **最佳实践**：方法用 `&self`（借用）而不是 `self`（移动所有权），这样调用后实例仍然可用。

---

## 第六章：枚举与模式匹配

### 6.1 定义枚举

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),   // 每个变体可携带不同类型的数据
    V6(String),
}

enum Message {
    Quit,                        // 无数据
    Move { x: i32, y: i32 },    // 命名字段（类似结构体）
    Write(String),               // 单个 String
    ChangeColor(i32, i32, i32), // 三个 i32
}
```

### 6.2 Option<T> — Rust 没有 null！

```rust
enum Option<T> {
    Some(T),
    None,
}

let some = Some(5);
let none: Option<i32> = None;   // 必须标注类型
```

> `Option<T>` 和 `T` 是**不同类型**，编译器强制你处理 `None` 情况。

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
// let sum = x + y;  // ❌ 编译错误！必须先 unwrap
```

### 6.3 match — 穷尽性匹配

```rust
enum Coin { Penny, Nickel, Dime, Quarter(UsState) }

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {    // 绑定值
            println!("来自 {:?} 的 25 分硬币！", state);
            25
        }
    }
}
```

> **match 必须覆盖所有可能**，否则编译报错。

### 6.4 if let 和 let...else

```rust
// if let —— 只关心一个分支，更简洁
if let Some(max) = config_max {
    println!("最大值是 {max}");
}

// let...else —— 匹配失败时 diverge（return/panic/break）
let Coin::Quarter(state) = coin else {
    return None;
};
// state 在外部作用域可用
```

> `if let` 牺牲了穷尽检查，换来简洁。`let...else` 让 happy path 保持外层作用域。

---

## 第七章：模块系统

### 7.1 包、Crate、模块

| 组件 | 说明 |
|------|------|
| Crate | 最小编译单元，产出一个库或可执行文件 |
| Package | Cargo 构建单元，包含一个或多个 Crate |
| Module | 组织代码、控制可见性 |

```
项目结构：
├── Cargo.toml
├── src/
│   ├── main.rs          # 二进制 crate 入口
│   ├── lib.rs           # 库 crate 入口
│   ├── garden.rs        # mod garden; 对应的文件
│   └── garden/
│       └── vegetables.rs
```

### 7.2 路径与可见性

```rust
mod front_of_house {
    pub mod hosting {           // pub mod 让祖先可访问
        pub fn add_to_waitlist() {}  // pub fn 让外部可调用
    }
    mod serving {               // 私有模块
        fn take_order() {}      // 私有函数
    }
}

// 绝对路径
crate::front_of_house::hosting::add_to_waitlist();

// 相对路径（从当前模块出发）
front_of_house::hosting::add_to_waitlist();
```

> **可见性不传递**：路径上每一段都必须 `pub`。

### 7.3 use 引入作用域

```rust
// 函数：引入父模块，调用时加模块名前缀（符合惯例）
use crate::front_of_house::hosting;
fn eat() { hosting::add_to_waitlist(); }

// 结构体/枚举：引入完整路径
use std::collections::HashMap;

// 重命名解决冲突
use std::fmt::Result;
use std::io::Result as IoResult;

// 嵌套路径
use std::{cmp::Ordering, io};
use std::io::{self, Write};

// pub use 重新导出（解耦内部结构与公开 API）
pub use crate::front_of_house::hosting;
```

> **关键规则**：`use` 只在当前作用域生效。子模块需要自己的 `use`。

---

## 第八章：常用集合

### 8.1 Vec<T> — 动态数组

```rust
let v: Vec<i32> = Vec::new();       // 空向量（需类型标注）
let v = vec![1, 2, 3];              // 宏，类型自动推断

let mut v = Vec::new();
v.push(5); v.push(6);               // 追加元素

// 读取
let third: &i32 = &v[2];            // 越界 → panic
let third: Option<&i32> = v.get(2); // 越界 → None

// 遍历
for i in &v { println!("{i}"); }
for i in &mut v { *i += 50; }       // 可变遍历
```

> **借用规则**：持有 `&v[0]` 后不能 `v.push()`，因为 push 可能 realloc 导致旧引用失效。

### 8.2 String — UTF-8 字符串

```rust
let mut s = String::new();
let s = "hello".to_string();        // or String::from("hello")

s.push_str(" world");               // 追加 &str
s.push('!');                        // 追加 char

// 拼接
let s1 = String::from("Hello");
let s2 = String::from(" world");
let s3 = s1 + &s2;                  // s1 被移动！
let s4 = format!("{s1}-{s2}");      // 推荐，不移动所有权
```

> **不能索引 String**：`s[0]` 编译错误！因为 UTF-8 字符长度 1-4 字节，索引语义不明确。

```rust
let hello = "Здравствуйте";
for c in hello.chars() { /* 逐个 Unicode 字符 */ }
for b in hello.bytes() { /* 逐个字节 */ }
```

### 8.3 HashMap<K, V>

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

// 读取
let score = scores.get(&"Blue").copied().unwrap_or(0);

// 遍历（无序）
for (key, value) in &scores { println!("{key}: {value}"); }

// 插入或忽略
scores.entry(String::from("Yellow")).or_insert(50);

// 更新已有值
let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

---

## 第九章：错误处理

### 9.1 panic! — 不可恢复错误

```rust
panic!("crash and burn");

let v = vec![1, 2, 3];
v[99];    // 隐式 panic
```

**调试**：`RUST_BACKTRACE=1 cargo run` 显示调用栈。

**生产环境配置**（在 Cargo.toml 中设置 abort 而非 unwind）：
```toml
[profile.release]
panic = 'abort'
```

### 9.2 Result<T, E> — 可恢复错误

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// match 处理
let file = match File::open("hello.txt") {
    Ok(file) => file,
    Err(e) => panic!("打开文件失败: {e:?}"),
};

// 快捷方法
let file = File::open("hello.txt").expect("打开文件失败");
let file = File::open("hello.txt").unwrap();  // 不带自定义消息
```

### 9.3 ? 操作符 —— 错误传播

```rust
fn read_username() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

> `?` 等价于：`Ok(val)` → 解包继续；`Err(e)` → 提前返回 `Err(e)`。

**`main` 也可以返回 Result**：
```rust
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    File::open("hello.txt")?;
    Ok(())
}
```

### 9.4 何时 panic 何时 Result？

| 情况 | 推荐做法 |
|------|---------|
| 调用者可以处理的错误（文件不存在、格式错误） | 返回 `Result` |
| 编程 bug（索引越界、违反契约） | `panic!` |
| 原型/测试代码 | `unwrap()` / `expect()` |

**验证型构造器模式**（用 panic 保证不变量）：
```rust
pub struct Guess { value: i32 }

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess 必须在 1-100 之间，得到 {value}");
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 { self.value }
}
```
> 私有字段 + 验证构造器 + getter = 所有拿到 `Guess` 的代码都知道值合法。

---
