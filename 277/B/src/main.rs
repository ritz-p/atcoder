use proconio::marker::Chars;
use proconio::input;

fn main(){
    input!{
        n:usize,
        strs:[Chars;n],
    };
    //A , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , T , J , Q , K
    for i in 0..n-2{
        for j in i+1..n{
            if strs[i] == strs[j]{
                println!("No");
                return;
            }
        }
    }
    for i in 0..n{
        if (strs[i][0] == 'H' || strs[i][0] == 'D' || strs[i][0] == 'C' || strs[i][0] == 'S') 
        && (strs[i][1] == 'A' || strs[i][1] == '2' || strs[i][1] == '3' || strs[i][1] == '4'
        || strs[i][1] == '5' || strs[i][1] == '6' || strs[i][1] == '7' || strs[i][1] == '8'
        || strs[i][1] == '9' || strs[i][1] == 'T' || strs[i][1] == 'J' || strs[i][1] == 'Q' 
        || strs[i][1] == 'K'){
        }else{
            println!("No");
            return;
        }
    }

    println!("Yes");

}