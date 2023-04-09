use proconio::input;
use std::collections::HashMap;
 
fn main() {
    input! {
        (_n,q):(u32,u32),
    }
 
    let mut operation: Vec<(u8, u32, u32)> = Vec::new();
    let mut S: HashMap<(u32, u32), bool> = HashMap::new();
    let mut answer = "".to_string();
 
    for _ in 0..q {
        input! {
            (t,a,b):(u8,u32,u32),
        }
 
        let t: u8 = t;
        let a: u32 = a;
        let b: u32 = b;
 
        operation.push((t, a, b));
    }
 
    for each in operation {
        let t = each.0;
        let a = each.1;
        let b = each.2;
 
        if t == 1 {
            S.insert((a, b), true);
        }
 
        if t == 2 {
            S.remove(&(a, b));
        }
 
        if t == 3 {
            if S.contains_key(&(a, b)) && S.contains_key(&(b, a)) {
                answer += "Yes\n";
            } else {
                answer += "No\n";
            }
        }
    }
    println!("{}", answer.trim());
}