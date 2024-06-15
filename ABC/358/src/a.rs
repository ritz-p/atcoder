use proconio::input;

fn main(){
    input!{
        s: String,
        t: String,
    };
    if s == "AtCoder".to_string() && t == "Land"{
        println!("Yes");
    }else{
        println!("No");
    }
}
