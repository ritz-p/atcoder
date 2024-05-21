use proconio::input;

fn main(){
    input!{
        n: usize
    };
    let mut res = n;

    for i in 1..=n{
        let s1 = i.to_string();
        let s2 = format!("{:o}",i);
        if s1.contains("7") || s2.contains("7"){
            res -= 1;
        }
    }
    println!("{}",res);
}
