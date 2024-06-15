use proconio::input;

fn main(){
    input!{
        n: usize,
        a: usize,
        mut t: [usize;n]
    };
    let mut current = 0;
    for (_index,e) in t.iter().enumerate(){
        current = current.max(*e);
        current += a;
        println!("{}",current);
    }
}
