use proconio::input;

fn main(){
    input!{
        n:usize,
        x:usize,
        arr:[usize;n],
    };

    if arr.iter().any(|e| *e == x){
        println!("Yes");
    }else{
        println!("No");
    }

}