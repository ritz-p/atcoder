use proconio::input;
use itertools::Itertools;
fn main(){
    input!{
        n: usize,
        q: usize,
    };
    let mut visited = vec![false;n+1];
    let mut recall = 1;
    let mut answer = vec![];
    for _ in 0..q{
        input!{
            task: usize,
        };
        if task == 1{
            continue;
        }else if task == 2{
            input!{
                x: usize,
            };
            visited[x] = true;
        }else if task == 3{
            while visited[recall]{
                recall += 1;
            }
            answer.push(recall);
        }
    }
    println!("{}", answer.iter().join("\n"));
}