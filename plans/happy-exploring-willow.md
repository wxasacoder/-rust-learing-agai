# Rust 学习路线规划

## Context
The user wants a fast "速通" (speed run) of the Rust book, followed by an onion-layer deep dive with small achievement milestones. They want to cover the book quickly first, then reinforce with hands-on exercises that build confidence progressively.

---

## Phase 1: 速通概览 (1-2 days)

**目标**: 快速理解 Rust 的核心概念，不求甚解，先建立整体认知框架

**方法**: 阅读官方书的关键章节，标记重点，对复杂概念做"先了解后深究"策略

### 核心章节速通 (按优先级排序)
1. **第 1-3 章**: 基础入门 (Hello World, 猜数字游戏, 通用概念)
2. **第 4 章**: 所有权 (OwnerShip) — Rust 的灵魂, 必须理解
3. **第 5-6 章**: 结构体和枚举
4. **第 7-9 章**: 模块化编程、常见集合、错误处理
5. **第 10 章**: 泛型和 Traits — Rust 的类型系统核心
6. **第 11 章**: 测试 (写代码必备)
7. **第 12 章**: 实战项目 I/O (猜数字进阶)
8. **第 13-15 章**: 迭代器、智能指针
9. **第 16-17 章**: 并发、面向对象特性

**跳过/后期再看**: 第 18 章 (模式匹配, 速通可略), 第 19 章 (高级特性, 洋葱第二层), 第 20 章 (Web 服务器)

---

## Phase 2: 洋葱第一层 — 小成就感练习

After the speed run, reinforce with progressively challenging exercises:

### Level 1: 基础手感 (Day 3-4)
- 实现一个计算器 CLI
- 实现 FizzBuzz 和斐波那契
- 实现一个简单的 TODO 命令行 (文件持久化)

### Level 2: 理解类型系统 (Day 5-7)
- 实现一个简单的 Result/Option 使用练习
- 写一个自定义 Trait 并实现
- 实现一个简单的链表 (理解所有权和引用)

### Level 3: 实用项目 (Week 2)
- 实现一个简单的 grep 工具 (minigrep)
- 实现一个简单的 HTTP 客户端
- 实现一个配置解析器 (读 JSON/TOML 文件)

### Level 4: 深入并发和异步 (Week 3+)
- 实现一个多线程的计数器
- 实现一个简单的异步任务调度器

---

## Key Files / Resources

- [Rust Book](https://doc.rust-lang.org/book/) — 官方教材
- This project directory: `/Users/wuxin/Documents/rust_work_space/rust-learing-again`

---

## Verification

1. User can read through book chapters and mark key concepts
2. Complete each exercise level sequentially
3. Build and run all exercise projects with `cargo run`
