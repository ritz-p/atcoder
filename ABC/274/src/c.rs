use std::io;
use proconio::input;

fn main(){
  input! {
      n: usize,
      ameba: [usize; n],
  }
  let mut res = vec![0;2*n+2];
  for i in 1..=n{
    let a = ameba[i-1];
    res[2*i] = res[a]+1;
    res[2*i+1] = res[a]+1;
  }
  for i in 1..2*n+2 {
    println!("{}",res[i]);
  }
}