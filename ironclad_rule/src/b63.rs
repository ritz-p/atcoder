use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        r: usize,
        c: usize,
        sy: usize,
        sx: usize,
        gy: usize,
        gx: usize,
        board: [Chars;r]
    };
    let mut map = vec![vec![usize::MAX;c];r];
    let mut visited = vec![vec![false;c];r];
    let mut current:Vec<(usize,usize)> = vec![(sy-1,sx-1)];
    map[sy-1][sx-1] = 0;
    loop{
        let mut points:Vec<(usize,usize)> = vec![];
        if current.is_empty(){
            return;
        }
        for (x,y) in &current{
            if board[*x][*y] != '.' || visited[*x][*y]{
                continue;
            }
            visited[*x][*y] = true;
            if *x == gy-1 && *y == gx-1{
                println!("{}",map[*x][*y]);
                return;
            }
            if x > &0 && board[x-1][*y] == '.'{
                points.push((x-1,*y));
                map[x-1][*y] = map[x-1][*y].min(map[*x][*y]+1);
            }
            if y > &0 && board[*x][*y-1] == '.'{
                points.push((*x,y-1));
                map[*x][*y-1] = map[*x][*y-1].min(map[*x][*y]+1);
            }
            if x < &(r) && board[x+1][*y] == '.'{
                points.push((x+1,*y));
                map[x+1][*y] = map[x+1][*y].min(map[*x][*y]+1);
            }
            if y < &(c) && board[*x][*y+1] == '.'{
                points.push((*x,y+1));
                map[*x][*y+1] = map[*x][*y+1].min(map[*x][*y]+1);
            }
        }
        current = points;
    }
}