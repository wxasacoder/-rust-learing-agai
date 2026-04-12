# Rust 速通笔记 · 第 10-17 章

> 涵盖：泛型与 Trait、测试、I/O 项目、函数式特性、Cargo、智能指针、并发、OOP

---

## 第十章：泛型、Trait 与生命周期

### 10.1 泛型

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}
```

> **泛型在编译时被单态化**（Monomorphization）：为每种具体类型生成专用代码，**零运行时开销**。

**泛型结构体**：
```rust
struct Point<T, U> { x: T, y: U }
let p = Point { x: 5, y: 4.0 };
```

**泛型枚举**（你已经用过了）：
```rust
enum Option<T> { Some(T), None }
enum Result<T, E> { Ok(T), Err(E) }
```

### 10.2 Trait — 共享行为

Trait 类似其他语言的接口。

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

**默认实现**：
```rust
pub trait Summary {
    fn summarize(&self) -> String { String::from("(Read more...)") }
}
impl Summary for NewsArticle {}   // 空实现 = 用默认
```

**Trait 作为参数**：
```rust
// impl Trait 语法（简洁）
fn notify(item: &impl Summary) { println!("{}", item.summarize()); }

// Trait bound 语法（可复用类型）
fn notify<T: Summary>(item: &T) { println!("{}", item.summarize()); }

// 多个 Trait
fn notify(item: &(impl Summary + Display)) { }

// where 子句（参数多时更清晰）
fn notify<T, U>(t: &T, u: &U) -> i32
where T: Display + Clone, U: Clone + Debug,
{ /* ... */ }
```

> **Orphan Rule**：只能为本地 Trait 实现本地类型，或为本地类型实现任何 Trait。不能为外部类型实现外部 Trait。

### 10.3 生命周期

生命周期确保引用不会超出被引用数据的存活范围。

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

> `'a` 表示返回值的生命周期和两个参数中**较短的那个**一样长。

**生命周期省略规则**（编译器自动推导）：
1. 每个引用参数各得一个生命周期
2. 如果只有一个输入生命周期，它被赋给所有输出生命周期
3. 如果有 `&self`，它的生命周期赋给所有输出生命周期

**结构体持有引用**：
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

> 大多数常见模式编译器能自动推导，只有返回值引用来源不明时才需要手动标注。

---

## 第十一章：测试

### 11.1 写测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // 自定义失败信息
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
            "问候语中没有包含名字，值是 `{result}`");
    }

    // 应该 panic 的测试
    #[test]
    #[should_panic(expected = "必须大于或等于 1")]
    fn invalid_guess() {
        Guess::new(200);
    }

    // Result 测试（可用 ?）
    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 { Ok(()) } else { Err("wrong".into()) }
    }
}
```

**测试宏对比**：

| 宏 | 用法 | 失败时输出 |
|---|------|----------|
| `assert!` | `assert!(条件)` | 只说条件为 false |
| `assert_eq!` | `assert_eq!(左, 右)` | 打印左右值 |
| `assert_ne!` | `assert_ne!(左, 右)` | 打印左右值 |

### 11.2 运行测试

```bash
cargo test                    # 运行所有测试（并行）
cargo test 名字                 # 按名字筛选（子串匹配）
cargo test -- --show-output   # 显示通过测试的输出
cargo test -- --test-threads=1  # 单线程运行（共享状态时安全）
```

### 11.3 测试组织

| | 单元测试 | 集成测试 |
|---|---|---|
| 位置 | 源码同文件，`mod tests` | `tests/` 目录 |
| 可见性 | 可测**私有**函数 | 只能测**公开** API |
| 编译 | `#[cfg(test)]` | Cargo 自动处理 |

---

## 第十二章：I/O 项目（minigrep）

### 项目结构（推荐模式）

```
minigrep/
├── Cargo.toml
├── src/
│   ├── main.rs    # 入口，处理 CLI 解析和错误
│   └── lib.rs     # 核心逻辑，可测试
└── tests/
    └── integration_test.rs
```

### 核心代码

```rust
// src/lib.rs
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>)
        -> Result<Config, &'static str>
    {
        args.next(); // 跳过程序名
        let query = args.next().ok_or("没拿到查询字符串")?;
        let file_path = args.next().ok_or("没拿到文件路径")?;
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results { println!("{line}"); }
    Ok(())
}

// src/main.rs
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(args.into_iter())
        .unwrap_or_else(|err| {
            eprintln!("参数解析失败: {err}");
            process::exit(1);
        });

    if let Err(e) = minigrep::run(config) {
        eprintln!("应用错误: {e}");
        process::exit(1);
    }
}
```

> **关键设计**：`main.rs` 只做协调，核心逻辑放 `lib.rs`。这样核心逻辑可测试，`main` 不可测。

---

## 第十三章：函数式特性

### 13.1 闭包

```rust
// 语法：|参数| 表达式
let add_one = |x| x + 1;
let add_one_explicit = |x: i32| -> i32 { x + 1 };

// 捕获环境变量
let equal_to_x = |z| z == x;   // 自动借用

// move 强制获取所有权（多线程必须用）
let x = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("这里有一个向量: {:?}", x);
});
```

**闭包的三种 `Fn` Trait**：

| Trait | 能力 | 调用次数 |
|-------|------|---------|
| `FnOnce` | 可移动捕获的值 | 一次 |
| `FnMut` | 可修改捕获的值 | 多次 |
| `Fn` | 只读捕获的值 | 多次 |

### 13.2 迭代器

```rust
// 创建迭代器
let v1 = vec![1, 2, 3];
let iter = v1.iter();          // &T
let into_iter = v1.into_iter(); // T（消耗所有权）

// 适配器（惰性，不调用消费方法就不执行）
v1.iter().map(|x| x + 1);       // 什么都不发生！
let doubled: Vec<_> = v1.iter().map(|x| x + 1).collect();  // 需要 collect

// filter
let shoes_in_size = shoes.into_iter()
    .filter(|s| s.size == shoe_size)
    .collect();
```

> **零成本抽象**：迭代器和手写 for 循环性能一样，编译器会优化到相同的汇编。

### 13.3 用迭代器改写 minigrep

```rust
// 之前：for 循环 + push
let mut results = Vec::new();
for line in contents.lines() {
    if line.contains(query) { results.push(line); }
}
results

// 之后：链式调用（更清晰、更函数式）
contents.lines()
    .filter(|line| line.contains(query))
    .collect()
```

---

## 第十四章：Cargo 进阶

### 14.1 Release Profile

```toml
# Cargo.toml
[profile.dev]
opt-level = 0          # 开发：快速编译

[profile.release]
opt-level = 3          # 发布：最优性能
panic = 'abort'        # 发布时 abort 而非 unwind（更小二进制）
```

### 14.2 发布到 crates.io

```toml
[package]
name = "my_crate"
version = "0.1.0"
edition = "2024"
description = "一个很棒的库"
license = "MIT OR Apache-2.0"
```

```bash
cargo doc --open          # 生成文档
cargo publish             # 发布（不可逆！）
cargo yank --vers 1.0.1   # 撤销（已有项目不受影响）
```

### 14.3 Workspace

```toml
# 根目录 Cargo.toml
[workspace]
resolver = "2"
members = ["adder", "add_one"]
```

> 共享 `target/` 和 `Cargo.lock`，编译更快，版本对齐。

```bash
cargo run -p adder        # 运行指定包
cargo test -p add_one     # 测试指定包
```

---

## 第十五章：智能指针

### 15.1 什么是智能指针？

普通引用 `&T` 只是地址。智能指针是**结构体 + `Deref` + `Drop`**，通常**拥有数据**。

### 15.2 Box<T>

```rust
let b = Box::new(5);      // 数据在堆上，指针在栈上
println!("b = {b}");
```

**用途**：递归类型
```rust
enum List {
    Cons(i32, Box<List>),  // Box 打破无限大小
    Nil,
}
```

### 15.3 Deref Trait

`*y` 被编译器重写为 `*(y.deref())`。智能指针通过 `Deref` 表现得像引用。

**Deref 强制转换**（自动，零开销）：
```rust
fn takes_str(s: &str) { }
let s = MyBox::from(String::from("hello"));
takes_str(&s);             // &MyBox<String> → &String → &str
```

### 15.4 Drop Trait

```rust
impl Drop for MyType {
    fn drop(&mut self) { /* 清理逻辑 */ }
}
```

> 不能直接调用 `drop()` 方法。用 `std::mem::drop(value)` 强制提前释放。

### 15.5 Rc<T> — 多所有者

```rust
use std::rc::Rc;

let a = Rc::new(Cons(5, Rc::new(Nil)));
let b = Cons(3, Rc::clone(&a));    // 引用计数 +1
let c = Cons(4, Rc::clone(&a));    // 引用计数 +1
// a 离开作用域时计数 -1，不会 drop，因为还有 b 和 c 引用它
```

> 仅单线程。多线程用 `Arc<T>`。

### 15.6 RefCell<T> — 内部可变性

```rust
use std::cell::RefCell;

let x = RefCell::new(5);
let val = x.borrow();       // 不可变借用（运行检查）
let mut val = x.borrow_mut(); // 可变借用
```

| 类型 | 借用检查时机 | 适用场景 |
|------|------------|---------|
| `Box<T>` | 编译时 | 单一所有者 |
| `Rc<T>` | 编译时 | 多只读所有者 |
| `RefCell<T>` | **运行时** | 编译器无法证明但你知道安全的场景 |

> 运行时违规 → panic（而非编译错误）。

**组合使用**：
```rust
Rc<RefCell<T>>    // 多所有者 + 内部可变性
Rc<T>             // 多只读所有者
RefCell<T>        // 单所有者 + 运行时可变借用
```

### 15.7 Weak<T> — 防止循环引用

```rust
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,     // 弱引用（不计数）
    children: RefCell<Vec<Rc<Node>>>, // 强引用（计数）
}

let weak = Rc::downgrade(&branch);   // 创建弱引用
if let Some(strong) = weak.upgrade() { /* 还活着 */ }
else { /* 已被 drop */ }
```

> **父子关系**：父→子用 `Rc`（强引用，拥有），子→父用 `Weak`（弱引用，不拥有）。

---

## 第十六章：并发

### 16.1 线程

```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(move || {     // move 转移所有权
    for i in 1..10 {
        println!("线程中的数字 {i}");
        thread::sleep(Duration::from_millis(1));
    }
});

handle.join().unwrap();    // 等待线程结束
```

> 主线程退出会**强制关闭**所有子线程。必须 `join()`。

### 16.2 消息传递（mpsc）

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

// 发送
tx.send(String::from("hi")).unwrap();

// 接收
let received = rx.recv().unwrap();       // 阻塞等待

// 迭代器接收（自动等待，通道关闭时停止）
for msg in rx { println!("收到: {msg}"); }
```

> `send()` **移动所有权**，发送后无法再使用原值。多生产者用 `tx.clone()`。

### 16.3 共享状态（Mutex）

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles { handle.join().unwrap(); }
println!("结果: {}", *counter.lock().unwrap());  // 10
```

> `MutexGuard` 离开作用域自动释放锁（Drop）。Rust 防止忘记解锁，但**无法防止逻辑死锁**。

### 16.4 Send 和 Sync

| Trait | 含义 | 例外 |
|-------|------|------|
| `Send` | 可跨线程转移所有权 | `Rc<T>`、裸指针 |
| `Sync` | `&T` 可被多线程引用 | `Rc<T>`、`RefCell<T>` |

> 大多数类型自动实现。编译器自动检查。

---

## 第十七章：面向对象特性

### 17.1 Rust 有 OOP 吗？

Rust 没有继承，但通过其他方式实现 OOP 的三大特性：

| OOP 特性 | Rust 的实现 |
|---------|-----------|
| 封装 | `pub` / 私有字段 + 方法 |
| 多态 | Trait + 泛型 / Trait 对象 |
| 继承 | **无**，用 Trait 默认方法 + 组合替代 |

### 17.2 Trait 对象 —— 运行时多态

```rust
pub trait Draw { fn draw(&self); }

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,   // 异质集合
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();              // 运行时分发
        }
    }
}
```

| 方案 | 分发方式 | 性能 | 灵活性 |
|------|---------|------|-------|
| 泛型 | 静态（编译时） | 最优 | 同质类型 |
| Trait 对象 | 动态（vtable） | vtable 开销 | 异质类型 |

### 17.3 状态模式 —— 类型状态模式

```rust
// 每个状态是一个类型，方法消耗 self 返回新类型
pub struct DraftPost { content: String }
pub struct PendingReviewPost { content: String }
pub struct Post { content: String }

impl Post {
    pub fn new() -> DraftPost { DraftPost { content: String::new() } }
    pub fn content(&self) -> &str { &self.content }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) { self.content.push_str(text); }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post { content: self.content }
    }
}
```

> **编译时保证**：不可能在 `DraftPost` 上调用 `content()`（方法不存在）。非法状态转换编译不通过。

---
