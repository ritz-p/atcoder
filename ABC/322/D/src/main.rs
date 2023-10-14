use proconio::input;

fn main(){
    input!{
        p: [[Chars;]]
    };
}

fn rotate(obj: mut Vec<Vec<char>>){
    let mut rotate_obj = obj;
    for i in 0..4{
        for j in 0..4{
            rotate_obj[3 - j][i] = obj[i][j];
        }
    }
    obj = rotate_obj;
}