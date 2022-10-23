use std::io;

fn main(){
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  let vec: Vec<&str> = line.split_whitespace().collect();
  let a:f32 = vec[0].trim().parse().unwrap_or(0.0);
  let b:f32 = vec[1].trim().parse().unwrap_or(0.0);
  let res:f32 = (b/a * 1000.0).round() / 1000.0;
  println!("{:.*}",3,res);
}