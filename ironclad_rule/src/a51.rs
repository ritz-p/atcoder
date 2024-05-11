use proconio::input;

fn main(){
    input!{
        n: usize,
    }
    let mut v = vec![];

    for _i in 0..n{
        input!{
            q: usize,
        }
        match q{
            1 => {
                input!{
                    s: String
                };
                v.push(s);
            },
            2 => {
                println!("{}",v[v.len()-1]);
            },
            3 => {
                v.pop();
            },
            _ => {}
        }
    }
}