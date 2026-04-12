# Rust 速通笔记 · 第 1-4 章

> 目标：用最短的篇幅，讲清楚 Rust Book 前四章的核心概念。每节附带最小可运行代码示例。

---

## 第一章：起步

### 1.1 工具链

```bash
rustc --version   # 编译器
cargo --version   # 构建工具 + 包管理器
```

### 1.2 Hello, World!

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn` 定义函数，`main` 是程序入口
- `println!` 是**宏**（注意感叹号），不是函数

### 1.3 Cargo 常用命令

```bash
cargo new my_project   # 创建新项目
cargo build            # 编译
cargo run              # 编译 + 运行
cargo check            # 只检查语法/类型，不生成二进制（更快）
```

---

## 第二章：猜数字游戏（实战入门）

完整代码：

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("猜一个 1-100 的数字！");

    loop {
        println!("请输入你的猜测：");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("读取失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal   => {
                println!("你赢了！");
                break;
            }
        }
    }
}
```

### 核心概念速查

| 概念 | 说明 |
|------|------|
| `let mut` | 可变变量（默认不可变） |
| `String::new()` | 空字符串，`::` 是关联函数 |
| `&mut guess` | 可变引用，让函数修改它 |
| `Result<T, E>` | `Ok` 或 `Err`，强制错误处理 |
| `match` | 模式匹配，必须覆盖所有分支 |
| Shadowing | `let guess: u32 = ...` 同名遮蔽，**可改类型** |
| `loop` / `break` / `continue` | 循环 / 退出 / 跳过 |

---

## 第三章：通用编程概念

### 3.1 变量与可变性

```rust
let x = 5;           // 不可变
let mut y = 5;       // 可变
y = 6;               // ✅
const MAX: u32 = 100_000;  // 常量，编译时求值，必须标类型
```

**Shadowing vs Mut**：

```rust
let mut s = "abc";
// s = 3;          // ❌ mut 不能改类型
let s = 3;         // ✅ shadowing 可以改类型
```

> 需要改值用 `mut`，需要改类型用 shadowing。

### 3.2 数据类型

#### 标量类型

| 类型 | 说明 | 示例 |
|------|------|------|
| `i32` | 有符号整数（**默认**） | `let x = 42;` |
| `u32` | 无符号整数 | `let x: u32 = 42;` |
| `f64` | 浮点数（**默认**） | `let x = 3.14;` |
| `bool` | 布尔 | `let t = true;` |
| `char` | Unicode 字符（4 字节） | `let c = '😻';` |

#### 复合类型

**Tuple** — 固定长度，可混合类型：
```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;   // 解构
let first = tup.0;     // 索引
```

**Array** — 固定长度，**同类型**，栈上分配：
```rust
let a = [1, 2, 3, 4, 5];
let b = [3; 5];        // [3, 3, 3, 3, 3]
// let bad = a[99];    // ❌ 运行时报 panic
```

### 3.3 函数

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b            // ← 无分号 = 表达式 = 返回值
}
```

> **语句 vs 表达式**：`let` 是语句（无返回值）；`5 + 6`、`{ }` 代码块是表达式。加分号 `;` 就把表达式变成了语句。

### 3.4 控制流

```rust
// if —— 条件必须是 bool！
if number % 3 == 0 {
    println!("能被 3 整除");
}

// if 可赋值，分支类型必须一致
let result = if number > 0 { "正数" } else { "非正数" };

// loop —— 可返回值
let value = loop {
    counter += 1;
    if counter == 10 { break counter * 2; }
};

// for —— 最推荐
for element in [10, 20, 30] { println!("{}", element); }
for n in (1..4).rev() { println!("{}!", n); }  // 3, 2, 1
```

---

## 第四章：所有权（Rust 的灵魂）

### 4.1 三条规则

1. 每个值都有一个**所有者**
2. 同一时间只能有**一个**所有者
3. 所有者离开作用域时，值会被**丢弃**（drop）

### 4.2 Move 与 Copy

```rust
let s1 = String::from("hello");
let s2 = s1;          // s1 被移动，失效！
// println!("{}", s1); // ❌

let s2 = s1.clone();  // 显式深拷贝，两者都有效
```

**哪些类型自动 Copy（不移动）？**
- 整数、浮点数、布尔、字符、纯 Copy 元组
- `String`、`Vec` 等堆类型 **不 Copy**

### 4.3 引用与借用

```rust
fn calc_len(s: &String) -> usize { s.len() }

let s1 = String::from("hello");
let len = calc_len(&s1);    // 不可变借用，s1 仍可用

fn change(s: &mut String) { s.push_str(", world"); }
let mut s = String::from("hello");
change(&mut s);             // 可变借用
```

### 4.4 引用的两条铁律

| 规则 | 说明 |
|------|------|
| **规则一** | 同一时刻：一个 `&mut` **或** 任意多个 `&` |
| **规则二** | 引用**始终有效**（不能悬垂） |

```rust
let mut s = String::from("hello");
let r1 = &s; let r2 = &s;       // ✅ 多个不可变引用
println!("{} {}", r1, r2);
let r3 = &mut s;                 // ✅ r1/r2 不再使用后，可变引用合法
```

> 编译器看的是**最后使用点**，不是作用域结束点（NLL）。

### 4.5 Slice（切片）

```rust
let s = String::from("hello world");
let hello = &s[0..5];      // "hello"
let full  = &s[..];        // 整个字符串

fn first_word(s: &str) -> &str { /* ... */ }
// 参数用 &str 而非 &String —— 可接受 &String、&str、字面量
```

---

## 速通总结

```
Rust 内存安全 = 所有权系统

变量默认不可变 ──→ 安全
         ↓
    需要修改？──→ let mut
         ↓
    String/Vec 等堆类型 ──→ 赋值 = Move
         ↓
    只是看看？──→ &T（不可变引用，可多个）
         ↓
    需要修改？──→ &mut T（可变引用，只能一个）
         ↓
    编译器在编译时检查 ──→ 零运行时开销，无 GC
```
