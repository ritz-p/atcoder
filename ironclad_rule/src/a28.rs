use proconio::input;

fn main(){
    input!{
        n: usize,
        ta: [(char,isize);n]
    };
    let mut current = 0;
    let r#mod = 10000;
    for (t,a) in ta{
        match t{
            '+' => {
                current += a;
            },
            '-' => {
                current -= a;
            },
            '*' => {
                current *= a;
            },
            _ => {}
        }
        if current < 0{
            current += r#mod;
        }
        current %= r#mod;
        println!("{}",current);
    }
}