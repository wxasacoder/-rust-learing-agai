# Rust 的 Copy 与 Move：传参时数据发生了什么

> 判断传参后原变量能否用，看 `Copy` trait，不是看栈还是堆。

---

## 一、两种行为

```rust
// 有 Copy：传参 = 拷贝，原变量还在
let a: i32 = 1;
foo(a);
println!("{}", a);  // ✅

// 没有 Copy：传参 = move，原变量失效
let s = String::from("hello");
foo(s);
// println!("{}", s);  // ❌ 所有权已转移
```

---

## 二、根本判断标准：`Copy` trait

```
实现了 Copy  → 传参按位拷贝（bitwise copy），原变量不动
没实现 Copy  → 传参 move（所有权转移），原变量标记不可用
```

**和栈/堆没有直接关系。** 即使一个类型的全部数据都在栈上，只要没实现 `Copy`，传参照样 move：

```rust
struct NoCopy {
    x: i32,   // 全在栈上，没有任何堆数据
}

fn main() {
    let n = NoCopy { x: 1 };
    foo(n);
    // n.x  // ❌ 所有权转移了
}
```

---

## 三、哪些类型有 Copy

| 有 `Copy` | 没有 `Copy` |
|-----------|-------------|
| `i32`, `u64`, `f64` 等整数/浮点 | `String` |
| `bool`, `char` | `Vec<T>`, `HashMap<K, V>` |
| `&T`（引用本身可复制） | `Box<T>`, `Rc<T>` |
| 元组（元素都 Copy 时） | 包含非 Copy 字段的结构体 |

**规律**：`drop` 时需要做任何事的类型（释放堆内存、关文件等），都不可能有 `Copy`。

---

## 四、Move 并不慢

`String`、`Vec` 的 move **只是拷贝栈上的元数据**（ptr + len + cap），堆上数据完全不动。move 的代价和 `Copy` 一样便宜，区别只在语义：一个允许再用，一个不允许。

```
move String 的过程：

  栈                           堆
┌─────────────┐              ┌─────────────┐
│ ptr ────────┼─────────────→│ hello       │
│ len: 5      │              └─────────────┐
│ cap: 5      │              ┌─────────────┘
└─────────────┘              │
                             │
  传参后，栈元数据拷贝给参数   │
┌─────────────┐              │
│ ptr ────────┼─────────────→│ (同一份数据)
│ len: 5      │              │
│ cap: 5      │              │
└─────────────┘              │
  原变量标记 invalid          │
┌─────────────┐              │
│ ptr ────────┼────── X      │
│ ...         │              │
└─────────────┘              │
```

---

## 五、`Copy` vs `Clone`

| | `Copy` | `Clone` |
|---|---|---|
| 触发方式 | 隐式（传参、赋值时自动） | 显式（必须调用 `.clone()`） |
| 复制方式 | 按位拷贝（bitwise） | 可自定义深拷贝逻辑 |
| 代价 | 总是便宜 | 可能昂贵（如深拷贝堆数据） |
| 关系 | `Copy` 隐含 `Clone` | `Clone` 不隐含 `Copy` |

```rust
// String 有 Clone 没有 Copy
let s1 = String::from("hello");
let s2 = s1.clone();   // ✅ 显式拷贝，s1 还在
// let s2 = s1;         // move，s1 失效

// i32 有 Copy
let a = 1;
let b = a;             // Copy 自动发生，a 还在
let c = a.clone();     // 也行（Copy 隐含 Clone），但没必要
```

---

## 六、如何给自定义类型加 Copy

```rust
#[derive(Copy, Clone)]   // Copy 要求所有字段都 Copy
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    foo(p);
    println!("{}, {}", p.x, p.y);  // ✅ 还在
}
```

如果字段包含非 `Copy` 类型，就不能派生 `Copy`：

```rust
struct User {
    name: String,  // String 没有 Copy
    age: i32,
}
// 不能 #[derive(Copy)] — 编译错误
```

---

## 七、完整可运行示例

```rust
fn main() {
    // 1. 基本类型（有 Copy）
    let a: i32 = 42;
    use_i32(a);
    println!("a 还在: {}", a);

    // 2. 引用（有 Copy）
    let s = String::from("hello");
    let r1 = &s;
    let r2 = r1;        // 引用本身被 Copy，不是 move
    println!("r1: {}, r2: {}", r1, r2);  // 两个都能用

    // 3. String（没有 Copy）
    let s1 = String::from("world");
    use_string(s1);     // move
    // println!("{}", s1);  // ❌ 已转移

    // 4. 想保留就用 clone
    let s3 = String::from("clone me");
    use_string(s3.clone());  // 拷贝一份传进去
    println!("s3 还在: {}", s3);
}

fn use_i32(x: i32) { println!("x = {}", x); }
fn use_string(s: String) { println!("s = {}", s); }
```

---

## 八、与 Java 的类比

| Java | Rust | 行为 |
|------|------|------|
| `int a = 1; foo(a)` | `let a: i32 = 1; foo(a)` | 值拷贝，原变量可用 |
| `String s = "hi"; foo(s)` | `let s = String::from("hi"); foo(s)` | ❌ 不一样！Java 拷贝引用，Rust move 所有权 |
| — | `foo(s.clone())` | Rust 想保留就得显式 clone |
| — | `foo(&s)` | Rust 用借用，零拷贝且不转移 |

> Java 的对象引用本质是指针，传参永远是指针拷贝。Rust 传一个 `String` 是传所有权，不是传指针——这是两个语言最大的差异。

---

"`★ Insight ───────────────────────────────────────────`
1. **别用栈/堆判断 move 还是 copy，用 `Copy` trait 判断**。即使全在栈上，没 `Copy` 就 move
2. **move 的代价和 copy 一样便宜**——只是元数据搬家，堆数据原地不动。Rust 的 move 不是 `memcpy` 整块数据
3. **引用 `&T` 本身是 `Copy` 的**——复制引用只是复制地址，不影响借用规则。多个引用指向同一份数据是正常操作
4. **Java 对象引用 ≠ Rust 拥有者类型**：Java 传 `String` 是传引用副本，Rust 传 `String` 是传所有权。要在 Rust 里模拟 Java 行为，用 `&String`/`&str`（借用）或 `.clone()`（显式拷贝）
`───────────────────────────────────────────────────────`
