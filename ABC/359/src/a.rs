use proconio::input;

fn main(){
    input!{
        n: usize,
        ss: [String;n]
    };
    let mut t = 0;
    for s in ss{
        if s == "Takahashi"{
            t += 1;
        }
    }

    println!("{}",t);
}
