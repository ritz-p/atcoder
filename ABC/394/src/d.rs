use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    };
    if s.len() % 2 == 1 {
        println!("No");
        return;
    }
    let mut stack = vec![];

    for &c in &s {
        match c {
            '(' | '[' | '<' => {
                stack.push(c);
            }
            ')' => {
                if stack.pop() != Some('(') {
                    println!("No");
                    return;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    println!("No");
                    return;
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    println!("No");
                    return;
                }
            }
            _ => {}
        }
    }
    if stack.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}
