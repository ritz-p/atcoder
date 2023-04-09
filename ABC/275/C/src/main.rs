use proconio::input;
use proconio::marker::Chars;
// (1,1),(1,2),(2,1),(2,2)

fn main(){
    input!{
        map: [Chars;9],
    };  

    let mut arr: Vec<(i32, i32)> = vec![];
    for i in 0..9{
        for j in 0..9{
            if map[i][j]=='#'{
                arr.push((i as i32,j as i32));
            }
        }
    }
    let mut count = 0;
    for i in 0..arr.len(){
        for j in 0..arr.len(){
            if i==j{
                continue;
            }
                // b と c は ad に対して線対称
            let a=arr[i];
            let b=arr[j];
            let x=b.0-a.0;
            let y=b.1-a.1;
            let c=(b.0-y,b.1+x);
            let d=(c.0-x,c.1-y);
            if arr.contains(&c) && arr.contains(&d){
                count+=1;
                // println!("a={:?}",a);
                // println!("b={:?}",b);
                // println!("c={:?}",c);
                // println!("d={:?}",d);
            }
        }
    }
    println!("{}",count/4);

}