use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String;n]
    };
    if n == 1{
        println!("Yes");
        return;
    }
    let mut b = true;
    for i in 1..n{
        if i == n-1{
            break;
        }
        if ss[i-1] == "sweet" && ss[i] == "sweet"{
            b = false;
            break;
        }
    }
    if b{
        println!("Yes");
    }else{
        println!("No");
    }
}
