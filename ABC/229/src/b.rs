use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let cs = (a + b).to_string();
    let d = a.max(b).to_string();
    if cs.len() > d.len() {
        println!("Hard");
        return;
    }

    for (index, c) in cs.chars().enumerate() {
        if d.chars().nth(index) > Some(c) {
            println!("Hard");
            return;
        }
    }
    println!("Easy");
}
