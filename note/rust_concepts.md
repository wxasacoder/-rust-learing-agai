# Rust 概念区分：Package / Crate / Module / Trait

## 关系概览

```
Workspace
  └── Package（包）        ← Cargo.toml 定义，可包含多个 Crate
        ├── Crate（编译单元）← 一个 main.rs 或 lib.rs 就是一个 Crate
        │     └── Module（模块）← .rs 文件，代码组织的基本单位
        │           └── Trait（特征）← 定义行为接口，类似其他语言的 interface
```

---

## 1. Package（包）

由 `Cargo.toml` 定义，是 Cargo 的概念。一个 package 可以包含一个或多个 crate。

```toml
# exercises/demo/Cargo.toml
[package]
name = "demo"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "exception_process"
path = "src/exception_process.rs"

[[bin]]
name = "container"
path = "src/container.rs"
```

运行指定 binary：`cargo run --bin exception_process -p demo`

---

## 2. Crate（编译单元）

Rust 最小的编译单元，分两种：

| 类型 | 入口文件 | 产物 |
|------|----------|------|
| binary | `src/main.rs` | 可执行文件 |
| library | `src/lib.rs` | 库（供其他 crate 引用） |

每个 `[[bin]]` 或默认的 `main.rs`/`lib.rs` 都是一个独立的 crate，彼此之间**不能互相访问**。

---

## 3. Module（模块）

crate 内部的代码组织方式，用 `mod` 声明。

```rust
// src/main.rs — 根模块
mod container;          // 引用 src/container.rs
mod exception_process;  // 引用 src/exception_process.rs

// 使用子模块中的内容
use container::MyContainer;
```

规则：
- 每个 `.rs` 文件默认就是一个 module
- 文件名 = 模块名（`foo.rs` → `mod foo`）
- 目录 `foo/mod.rs` 也对应 `mod foo`
- 默认 private，需用 `pub` 暴露

---

## 4. Trait（特征）

定义行为的接口，类似其他语言的 `interface`。

```rust
// 定义 trait
trait Pay {
    fn pay(&self, amount: &Decimal);           // 实例方法
    fn calculate(price: &Decimal, n: &Decimal) -> Decimal; // 静态方法
}

// 为类型实现 trait
impl Pay for UserId {
    fn pay(&self, amount: &Decimal) {
        println!("支付了 {}", amount);
    }
    fn calculate(price: &Decimal, n: &Decimal) -> Decimal {
        price.mul(n)
    }
}

// 使用
let user = UserId::from(1);
user.pay(&Decimal::from(100));             // 实例方法
UserId::calculate(&price, &count);         // 静态方法
```

---

## 一句话总结

- **Package** — Cargo 管理的工程单元（`Cargo.toml`）
- **Crate** — Rust 最小的编译单元（产出 binary 或 library）
- **Module** — crate 内部的代码组织方式（`.rs` 文件）
- **Trait** — 定义行为接口的抽象契约（`trait` 关键字）
