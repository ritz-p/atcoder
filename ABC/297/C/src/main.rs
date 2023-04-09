use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        h: usize,
        w: usize,
        mut map: [Chars;h],
    };
    let mut i = 0;
    while i < h{
        let mut j = 0;
        while j < w-1{
            if map[i][j] == 'T' && map[i][j+1] == 'T'{
                map[i][j] = 'P';
                map[i][j+1] = 'C';
                j += 2;
            }else{
                j+=1;
            }
        }
        i += 1;
    }
    for i in 0..h{
        for j in 0..w{
            print!("{}",map[i][j]);
        }
        print!("\n");
    }
}
