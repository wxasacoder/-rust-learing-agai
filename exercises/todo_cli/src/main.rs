use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const TODO_FILE: &str = "todos.json";

/// 简单的 TODO 项
#[derive(Debug)]
struct Todo {
    id: u32,
    text: String,
    done: bool,
}

impl Todo {
    fn new(id: u32, text: String) -> Self {
        Self {
            id,
            text,
            done: false,
        }
    }
}

/// 简单的 JSON 序列化 (手动实现, 不依赖外部库, 学习用)
fn todos_to_json(todos: &[Todo]) -> String {
    let items: Vec<String> = todos
        .iter()
        .map(|t| {
            format!(
                r#"{{"id":{},"text":"{}","done":{}}}"#,
                t.id,
                t.text.replace('"', r#"\""#),
                t.done
            )
        })
        .collect();
    format!("[{}]", items.join(","))
}

/// 简单的 JSON 反序列化 (手动实现, 学习用)
fn todos_from_json(json: &str) -> Vec<Todo> {
    let mut todos = Vec::new();
    let trimmed = json.trim();
    if trimmed == "[]" || trimmed.is_empty() {
        return todos;
    }

    // 去掉外层方括号
    let inner = &trimmed[1..trimmed.len() - 1];

    // 简单分割: 按顶层的 "}," 分割
    let mut items = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    for ch in inner.chars() {
        match ch {
            '{' => {
                depth += 1;
                current.push(ch);
            }
            '}' => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 => {
                items.push(current.clone());
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        items.push(current);
    }

    let mut next_id: u32 = 1;
    for item in &items {
        let id = extract_number(item, "id").unwrap_or(next_id) as u32;
        let text = extract_string(item, "text").unwrap_or_default();
        let done = extract_bool(item, "done");
        todos.push(Todo { id, text, done });
        if id >= next_id {
            next_id = id + 1;
        }
    }

    todos
}

fn extract_number(json: &str, key: &str) -> Option<u32> {
    let search = format!(r#""{}":"#, key);
    let start = json.find(&search)? + search.len();
    let remaining = &json[start..];
    let end = remaining.find(|c: char| !c.is_ascii_digit() && c != '-')?;
    remaining[..end].parse().ok()
}

fn extract_string(json: &str, key: &str) -> Option<String> {
    let search = format!(r#""{}":""#, key);
    let start = json.find(&search)? + search.len();
    let remaining = &json[start..];
    let mut result = String::new();
    let mut escaped = false;
    for ch in remaining.chars() {
        if escaped {
            result.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            return Some(result);
        } else {
            result.push(ch);
        }
    }
    None
}

fn extract_bool(json: &str, key: &str) -> bool {
    let search = format!(r#""{}":"#, key);
    if let Some(start) = json.find(&search) {
        let remaining = &json[start + search.len()..];
        return remaining.starts_with("true");
    }
    false
}

fn load_todos() -> Vec<Todo> {
    if Path::new(TODO_FILE).exists() {
        match fs::read_to_string(TODO_FILE) {
            Ok(content) => todos_from_json(&content),
            Err(_) => Vec::new(),
        }
    } else {
        Vec::new()
    }
}

fn save_todos(todos: &[Todo]) {
    let json = todos_to_json(todos);
    fs::write(TODO_FILE, json).expect("无法保存 TODO 文件");
}

fn next_id(todos: &[Todo]) -> u32 {
    todos.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

fn add_todo(todos: &mut Vec<Todo>, text: &str) {
    let id = next_id(todos);
    todos.push(Todo::new(id, text.to_string()));
    save_todos(todos);
    println!("已添加 TODO #{}: {}", id, text);
}

fn list_todos(todos: &[Todo]) {
    if todos.is_empty() {
        println!("TODO 列表为空");
        return;
    }
    println!("\n=== TODO 列表 ===");
    for todo in todos {
        let status = if todo.done { "[x]" } else { "[ ]" };
        println!("  #{} {} {}", todo.id, status, todo.text);
    }
    println!("==================\n");
}

fn done_todo(todos: &mut Vec<Todo>, id: u32) {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.done = true;
        save_todos(todos);
        println!("TODO #{} 已标记为完成", id);
    } else {
        println!("未找到 TODO #{}", id);
    }
}

fn remove_todo(todos: &mut Vec<Todo>, id: u32) {
    let len = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() < len {
        save_todos(todos);
        println!("TODO #{} 已删除", id);
    } else {
        println!("未找到 TODO #{}", id);
    }
}

fn print_help() {
    println!(
        r#"
用法: todo <命令> [参数]

命令:
  add <文本>    添加新的 TODO
  list          列出所有 TODO
  done <ID>     标记 TODO 为完成
  remove <ID>   删除 TODO
  help          显示此帮助
"#
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // 交互式模式
        println!("=== TODO 管理器 (交互式) ===");
        println!("输入 'help' 查看可用命令\n");

        loop {
            print!("todo> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit") {
                println!("再见!");
                break;
            }

            process_command(input);
        }
    } else {
        // 命令行参数模式
        let full_command = args[1..].join(" ");
        process_command(&full_command);
    }
}

fn process_command(input: &str) {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let command = parts[0].to_lowercase();
    let arg = parts.get(1).map(|s| *s).unwrap_or("");

    match command.as_str() {
        "add" => {
            if arg.is_empty() {
                println!("请提供 TODO 内容: add 内容");
                return;
            }
            let mut todos = load_todos();
            add_todo(&mut todos, arg);
        }
        "list" => {
            let todos = load_todos();
            list_todos(&todos);
        }
        "done" => {
            if let Ok(id) = arg.parse::<u32>() {
                let mut todos = load_todos();
                done_todo(&mut todos, id);
            } else {
                println!("请提供有效的 ID: done <ID>");
            }
        }
        "remove" => {
            if let Ok(id) = arg.parse::<u32>() {
                let mut todos = load_todos();
                remove_todo(&mut todos, id);
            } else {
                println!("请提供有效的 ID: remove <ID>");
            }
        }
        "help" => print_help(),
        "q" | "quit" => {
            println!("再见!");
            std::process::exit(0);
        }
        _ => println!("未知命令: '{}'. 输入 'help' 查看可用命令", command),
    }
}
