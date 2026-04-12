# Rust 字符串三剑客：`String`、`&str`、`&String`

> 这三个类型是 Rust 新手最容易混淆的地方。它们的核心区别不在"字符串"本身，而在**内存布局**和**所有权**。

---

## 一、先看内存布局

### 1. `String` — 堆上拥有者

```rust
let s: String = String::from("hello");
```

```
  栈（String 结构体，24 字节）            堆
┌─────────────────────┐              ┌─────────────────┐
│ ptr ────────────────────→            │ h e l l o \0    │
│ len: 5                │              └─────────────────┘
│ capacity: 5           │
└─────────────────────┘

字段含义：
  ptr       → 指向堆上 UTF-8 字节序列的指针
  len       → 当前字符串长度（字节数），5
  capacity  → 堆上已分配的总容量，5（还能塞多少字节而不重新分配）
```

**关键点**：`String` 拥有堆上数据的所有权，`drop` 时负责释放内存。

---

### 2. `&str` — 字符串切片（胖指针）

```rust
let s: &str = "hello";          // 字面量，数据在二进制只读段
let s2: &str = &some_string;    // 借用某个 String 的切片
```

```
情况 A：字符串字面量
  &str (栈，16 字节)              只读数据段（编译期嵌入）
┌───────────────┐               ┌─────────────────┐
│ ptr ──────────────→             │ h e l l o \0    │
│ len: 5          │               └─────────────────┘
└───────────────┘

情况 B：借用 String 的切片
  &str (栈，16 字节)              堆（由某个 String 拥有）
┌───────────────┐               ┌─────────────────┐
│ ptr ──────────────→             │ h e l l o \0    │
│ len: 5          │               └─────────────────┘
└───────────────┘
```

**关键点**：`&str` 是**胖指针**（fat pointer），栈上只存 (ptr, len) 两个字段，共 16 字节。它不拥有数据，不负责释放。

---

### 3. `&String` — 对拥有者的引用

```rust
let s = String::from("hello");
let r: &String = &s;
```

```
  &String (栈，8 字节)
┌─────────┐
│ ptr ────→ ┌─────────────────────┐              堆
└─────────┘ │ String (栈，24 字节) │           ┌─────────────────┐
            │ ptr ────────────────────→         │ h e l l o \0    │
            │ len: 5                │           └─────────────────┘
            │ capacity: 5           │
            └─────────────────────┘

指向链路：&String → String 结构体 → 堆上数据
需要两次解引用才能到达实际字符数据
```

**关键点**：`&String` 只是指向 `String` 结构体本身的指针，8 字节（普通指针大小）。它不直接指向字符数据。

---

## 二、三者对比总表

| 维度 | `String` | `&str` | `&String` |
|---|---|---|---|
| **大小** | 24 字节 | 16 字节 | 8 字节 |
| **所有权** | 拥有数据 | 借用数据 | 借用拥有者 |
| **数据位置** | 堆上 | 任意（只读段/堆/栈） | 堆上（通过 String） |
| **可修改** | 是（`mut` 时） | 否 | 否 |
| **类型本质** | 结构体 `Vec<u8>` 的包装 | DST `str` 的引用 | `String` 的引用 |
| **能否 `push`** | 能 | 不能 | 不能 |
| **能否传递** | 转移所有权 | 零拷贝借用 | 借用拥有者 |
| **常见程度** | 常用 | 常用 | 几乎不用 |

---

## 三、为什么 `&String` 几乎没用

### 场景：写一个函数，接受字符串参数

```rust
// ❌ 糟糕的设计：只接受 &String
fn greet_bad(s: &String) {
    println!("Hello, {}!", s);
}

greet_bad(&String::from("Alice"));  // OK
greet_bad(&"Bob".to_string());      // OK
greet_bad("Charlie");               // 编译错误！&str ≠ &String

// ✅ 正确的设计：接受 &str
fn greet_good(s: &str) {
    println!("Hello, {}!", s);
}

greet_good(&String::from("Alice"));  // OK（Deref 强转）
greet_good("Charlie");               // OK
greet_good(&some_string[0..3]);      // OK（切片也能传）
```

**`&String` 的问题**：它要求调用者必须持有 `String` 的所有权才能取引用，排除了 `&str` 字面量和其他字符串类型。

**`&str` 的优势**：通过 Rust 的 `Deref` 机制，`&String` 可以自动转为 `&str`，所以 `&str` 参数**通吃**所有字符串来源。

---

## 四、转换关系

```
                .to_string() / String::from()
     &str ───────────────────────────────────→ String
       ←────────────────────────────────────
            .as_str() / &*s / Deref 强转


                & 引用
     String ────────────────────────────────→ &String
                 （很少用，通常不需要）


                Deref<Target=str>
     &String ──────────────────────────────→ &str
             （编译器自动完成，无需显式调用）
```

```rust
let literal: &str = "hello";
let owned: String = literal.to_string();     // &str → String
let borrowed: &str = owned.as_str();         // String → &str
let ref_string: &String = &owned;            // String → &String（很少用）
let auto: &str = ref_string;                 // &String → &str（自动）
```

---

## 五、实战指南：什么时候用什么

```rust
// 需要拥有数据（存进结构体、跨作用域存活）→ String
struct User {
    name: String,    // 拥有数据，生命周期独立
}

// 只读视图（函数参数、返回值、临时访问）→ &str
fn print_name(name: &str) {
    println!("{}", name);
}

// 需要修改内容 → &mut String 或 &mut str
fn append_exclamation(s: &mut String) {
    s.push('!');
}

// 永远不要 → &String（没有合理的使用场景）
```

---

## 六、完整可运行示例

```rust
fn main() {
    // 1. String —— 拥有者
    let owned: String = String::from("hello");
    println!("owned: {}, size on stack: {} bytes", owned, std::mem::size_of::<String>());

    // 2. &str —— 字符串切片
    let slice: &str = &owned;
    println!("slice: {}, size on stack: {} bytes", slice, std::mem::size_of::<&str>());

    // 3. &String —— 对拥有者的引用
    let ref_string: &String = &owned;
    println!("ref_string: {}, size on stack: {} bytes", ref_string, std::mem::size_of::<&String>());

    // 4. &String 自动转为 &str
    let auto_slice: &str = ref_string;
    println!("auto_slice: {}", auto_slice);

    // 5. 验证它们指向同一份堆数据
    println!("slice ptr: {:p}", slice.as_ptr());
    println!("owned ptr: {:p}", owned.as_ptr());
    println!("same data: {}", slice.as_ptr() == owned.as_ptr());
}
```

输出：
```
owned: hello, size on stack: 24 bytes
slice: hello, size on stack: 16 bytes
ref_string: hello, size on stack: 8 bytes
auto_slice: hello
slice ptr: 0x...
owned ptr: 0x...
same data: true
```

---

## 七、与 Go 的类比（辅助理解）

| Rust | Go（近似） | 说明 |
|---|---|---|
| `String` | `[]byte` | 拥有堆上数据，可修改 |
| `&str` | `string` | 只读视图，ptr + len |
| `&String` | 无直接对应 | Go 没有这个等价物 |

> 注意：Go 的 `string` 本身就是 ptr+len 结构，更像 Rust 的 `&str`。Go 没有 Rust 这样严格的所有权区分。

---

## 八、深入理解胖指针（Fat Pointer）

`&str` 的 16 字节从何而来？这就涉及到 Rust 的**胖指针**概念。

### 普通指针 vs 胖指针

```rust
use std::mem::size_of;

fn main() {
    // 普通指针（thin pointer）— 只存一个地址
    println!("&i32      = {}", size_of::<&i32>());       // 8
    println!("&String   = {}", size_of::<&String>());    // 8
    println!("Box<i32>  = {}", size_of::<Box<i32>>());   // 8

    // 胖指针（fat pointer）— 地址 + 元数据
    println!("&str      = {}", size_of::<&str>());       // 16
    println!("&[i32]    = {}", size_of::<&[i32]>());     // 16
    println!("Box<[i32]>= {}", size_of::<Box<[i32]>>()); // 16
    println!("&dyn Any  = {}", size_of::<&dyn Any>());   // 16
}
```

### 为什么需要胖指针？

**普通指针**只存一个地址，够用是因为编译期已经知道目标类型的大小：

```rust
let x: i32 = 42;
let p: &i32 = &x;
// 编译器知道 i32 是 4 字节，一个地址就够了
```

但有些类型在编译期**大小未知**，运行时必须携带额外信息：

```rust
let arr = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr[1..4];
// 编译期不知道 slice 有多长，必须在运行时携带 len
```

### Rust 的两种胖指针

#### 1. 切片引用：`&[T]` 和 `&str`

```
胖指针结构（16 字节）
┌─────────┐
│  ptr    │  → 指向数据起始地址
│  len    │  → 元素个数（&str 中是字节数）
└─────────┘
```

```rust
let arr = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr[1..4];

// slice 的内存表示：
//   ptr → &arr[1] 的地址
//   len = 3
```

**为什么需要 `len`？** 因为切片在编译期长度未知，运行时必须携带长度信息才能知道能访问多少元素，也才能做边界检查。

#### 2. Trait 对象：`&dyn Trait` / `Box<dyn Trait>`

```
胖指针结构（16 字节）
┌──────────┐
│  ptr     │  → 指向具体数据
│  vtable  │  → 虚函数表指针（包含方法地址列表）
└──────────┘
```

```rust
trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) { println!("woof"); }
}

let dog = Dog;
let animal: &dyn Animal = &dog;

// animal 的内存表示：
//   ptr    → &dog 的地址
//   vtable → [Dog 的 speak 方法地址, ...]
```

**为什么需要 `vtable`？** 因为编译期不知道具体是什么类型，运行时必须通过虚表进行动态派发，才能找到对应类型的方法实现。

### 胖指针大小总览

| 类型 | 指针类型 | 大小 | 额外存了什么 |
|---|---|---|---|
| `&T`（已知大小） | 瘦指针 | 8 字节 | 无 |
| `&str` | 胖指针 | 16 字节 | 长度 |
| `&[T]` | 胖指针 | 16 字节 | 长度 |
| `&dyn Trait` | 胖指针 | 16 字节 | vtable |
| `Box<T>`（已知大小） | 瘦指针 | 8 字节 | 无 |
| `Box<dyn Trait>` | 胖指针 | 16 字节 | vtable |

### 回到 `&str`

`&str` 之所以是胖指针，因为 `str` 本身是**动态大小类型**（DST, Dynamically Sized Type）。编译器无法在编译期知道字符串有多长，所以 `&str` 必须在运行时携带 `len` 字段。

```
&str（16 字节）                    数据（任意位置）
┌───────────────┐                ┌───────────────────┐
│ ptr ─────────────→              │ h e l l o \0 ...  │
│ len: 5        │                └───────────────────┘
└───────────────┘
```

这也解释了为什么不能直接写 `let x: str = "hello";` — `str` 是 DST，编译期不知道大小，无法在栈上分配。必须通过胖指针 `&str`、所有权包装 `String`、或 `Box<str>` 来间接使用。

---

## 九、切片的本质：创造胖指针

> **切片操作不是复制数据，而是编译器帮你造一个新的胖指针 `(ptr, len)`。**

### 从字面量创建 `&str`

```rust
let a: &str = "hello";
```

```
  a（栈，16 字节）                   .rodata（二进制只读数据段）
┌──────────────────────┐           ┌──────────────────────┐
│ ptr ────────────────────→          │ 68 65 6c 6c 6f       │
│     指向 rodata 地址             │  h  e  l  l  o       │
│ len: 5               │           └──────────────────────┘
└──────────────────────┘
```

数据在编译期就嵌入到了可执行文件的 `.rodata` 段，只读，程序启动就存在。`a` 只是栈上的一个胖指针，记录了数据的起点和长度。

---

### 切片操作：取前 3 个字符

```rust
let a: &str = "hello";
let b: &str = &a[0..3];    // "hel"
```

```
  a（栈，16 字节）                          b（栈，16 字节）
┌──────────────────────┐                ┌──────────────────────┐
│ ptr ────────────────────→                │ ptr ────────────────────→
│     指向 'h' 的起始地址                 │     指向 'h' 的起始地址（同一个！）
│ len: 5               │                │ len: 3               │
└──────────────────────┘                └──────────────────────┘
          ↓                                      ↓
      .rodata（只读）
      ┌──────────────────────┐
      │ 68 65 6c 6c 6f        │
      │ h  e  l  l  o         │
      └──────────────────────┘
      ↑              ↑
      b.len=3，只认前3个
      a.len=5，认全部5个
```

**关键观察：**

- `b.ptr` 和 `a.ptr` **指向同一个地址**
- 唯一区别：`b.len = 3`，`a.len = 5`
- **没有复制任何数据**，只是新造了一个胖指针

**切片的本质就是：创造一个新的胖指针，修改其中的 `ptr` 和 `len`，指向原数据的一个子范围。**

---

### 切片也可以偏移起始位置

```rust
let a: &str = "hello";
let c: &str = &a[2..5];    // "llo"
```

```
  a（栈，16 字节）                          c（栈，16 字节）
┌──────────────────────┐                ┌──────────────────────┐
│ ptr ────────────────────→                │ ptr ────────────────────→
│     指向 'h' 起始                      │     指向第一个 'l'（偏移了2）
│ len: 5               │                │ len: 3               │
└──────────────────────┘                └──────────────────────┘
          ↓                                      ↓
      .rodata
      ┌──────────────────────┐
      │ 68 65 6c 6c 6f        │
      │ h  e  l  l  o         │
      └──────────────────────┘
            ↑              ↑
            a.ptr           c.ptr（偏移 2 字节）
                            c.len = 3
```

这里 `c.ptr = a.ptr + 2`，起始地址向后偏移了 2 个字节，长度变为 3。

---

### 切片操作的完整过程

```rust
let s = String::from("hello world");
let slice: &str = &s[6..11];    // "world"
```

```
编译器做的事：

1. 你写 &s[6..11]
2. 编译器检查 6..11 是合法 UTF-8 边界
3. 生成胖指针: FatPtr { ptr: s.as_ptr() + 6, len: 5 }
4. 返回这个新的 &str 值

运行时内存里没有"切片对象"，只有两个字段。
```

---

### 数组切片同理

```rust
let arr = [10, 20, 30, 40, 50];
let slice: &[i32] = &arr[1..4];    // [20, 30, 40]
```

```
  arr（栈，20 字节）                     slice（栈，16 字节）
┌─────────────────────────┐           ┌──────────────────────┐
│ 10 │ 20 │ 30 │ 40 │ 50  │           │ ptr → &arr[1] (20)   │
└─────────────────────────┘           │ len: 3               │
       ↑                    ↑          └──────────────────────┘
       slice.ptr            到这里结束，只认 3 个元素
```

`&[T]` 和 `&str` 的胖指针结构完全一样：`(ptr, len)`，只是 `&str` 多了一个 UTF-8 合法性约束。

---

### ⚠️ 切片是按字节切的

`&str` 的索引是**字节索引**，不是字符索引。对纯 ASCII 没问题，对多字节 UTF-8 字符必须切在边界上：

```rust
let s = "hello";
let first = &s[0..3];     // "hel" — OK，纯 ASCII

let s2 = "你好世界";
// "你好世界" 的 UTF-8 编码：
//  你: e4 bd a0  (3字节)
//  好: e5 a5 bd  (3字节)
//  世: e4 b8 96  (3字节)
//  界: e7 95 8c  (3字节)

let first = &s2[0..3];    // "你" — OK，切在 UTF-8 边界
let bad   = &s2[0..2];    // ❌ panic! e4 bd 不是合法 UTF-8
```

这也是为什么 `&str` 不能像 `Vec` 那样用 `.get(index)` 按索引访问字符——UTF-8 编码下每个字符的字节数不同，"第 N 个字符"需要从头扫描。

---

### 切片总结

| 你写的代码 | 编译器生成的胖指针 | 效果 |
|---|---|---|
| `let a: &str = "hello"` | `{ ptr: rodata_addr, len: 5 }` | 指向只读数据 |
| `&a[0..3]` | `{ ptr: a.ptr, len: 3 }` | 同一起点，更短 |
| `&a[2..5]` | `{ ptr: a.ptr + 2, len: 3 }` | 偏移起点 |
| `&s[6..11]`（String） | `{ ptr: s.ptr + 6, len: 5 }` | 指向堆上某一段 |

**一句话：切片操作 = 创造新的胖指针，不复制数据，零开销。**

---

"`★ Insight ───────────────────────────────────────────`
1. **`&str` 是最通用的字符串视图类型**：它直接指向字符数据，不关心数据在哪、谁拥有。函数参数优先用 `&str`
2. **`String` 是生命周期管理者**：需要跨作用域存活或修改内容时用，代价是堆分配
3. **`&String` 是冗余层**：它指向的是"管理者"而不是"数据"，既不能修改也不拥有，唯一的用途就是被编译器自动转成 `&str`
4. **胖指针 = 地址 + 运行时元信息**：切片胖指针存长度，trait 对象胖指针存 vtable，都是编译期信息不足时的必然产物
5. **切片的本质是造胖指针**：`&arr[start..end]` 不复制数据，只是创建新的 `(ptr, len)` 对，ptr 指向起点，len 告诉范围，零开销
`───────────────────────────────────────────────────────`"
