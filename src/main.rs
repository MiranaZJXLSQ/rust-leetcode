use std::fmt;

fn main() {
    println!("Hello, world!");
    // 这种大括号的姿势需要实现fmt::Display接口
    println!("student {}", Student(3));
    // 这种 :? 的姿势需要实现Debug接口
    println!("student {:?}", Student(3));
    println!("parse_bool_expr {}", parse_bool_expr(String::from("f")))
}

#[derive(Debug)]
struct Student(i32);

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        write!(f, "{}", self.0)
    }
}

pub fn parse_bool_expr(expression: String) -> bool {
    if expression.eq("t") {
        return true;
    }
    if expression.eq("f") {
        return false;
    }
    let mut stack = vec![];
    for c in expression.as_bytes() {
        if *c != b',' && *c != b')' {
            stack.push(*c);
        } else if *c == b')' {
            let (mut true_count, mut false_count) = (0, 0);
            while !stack.is_empty() {
                let temp = stack.pop().unwrap();
                if temp == b'(' {
                    break;
                } else if temp == b't' {
                    true_count += 1;
                } else if temp == b'f' {
                    false_count += 1;
                }
            }
            let operator = stack.pop().unwrap();
            if operator == b'!' {
                if false_count == 1 {
                    stack.push(b't');
                } else {
                    stack.push(b'f');
                }
            } else if operator == b'&' {
                if false_count == 0 {
                    stack.push(b't');
                } else {
                    stack.push(b'f');
                }
            } else if operator == b'|' {
                if true_count > 0 {
                    stack.push(b't');
                } else {
                    stack.push(b'f');
                }
            }
        }
    }
    let result = stack.pop().unwrap();
    if result == b't' { true } else { false }
}