# 型や関数メモ

## alphabet 作成

    ```rust
    let alphabet = ('a'..='z').collect::<Vec<char>>();
    ```

## sort

    - 昇順

    ```rust
    v.sort();
    ```

    - 降順

    ```rust
    v.sort();
    v.reverse();
    ```

    - tuple.0 を昇順、tuple.1 を降順

    ```rust
    v.sort_by(|a,b| {
        let ord = a.0.cmp(&b.0);
        if ord == std::cmp::Ordering::Equal {
            b.1.cmp(&a.1)
        }else{
            ord
        }
    });
    ```

## permutation

### permutations()

    - 被りありで同じ値が一度のみでてくる(0,1),(1,0)

    ```rust
    v.permutatios(v.len())
    ```

### permutations().unique()

    - (0,0,1,1) などで同じ値が複数あるときに並びが同じとき重複をなくす

    ```
    (0,0,1,1)
    (0,0,1,1) <- ここの重複をなくす
    (0,1,0,1)
    ```

    ```rust
    v.permutation(v.len()).unique()
    ```

## f64

### floor

    - 小数点以下切り捨て

    ```rust
    let mut f = 0.9;
    f = f.floor();
    ```

## 標準出力

    - 標準出力する

    ```rust
    let s = 100;
    println!("{}",res);
    ```

    - 小数点以下の桁数指定での標準出力

    ```rust
    let res = 0.0001;
    println!("{:.4}",res);
    ```
    　- 四捨五入されるので必要に応じて floor など使う

## HashSet

### new()

    - 空の HashSet を作る

    ```
    let mut set = HashSet::new();
    ```

### from_iter()

    - Vec から HashSet を生成する

    ```rust
    let v = vec![0,1,2];
    let set: HashSet<usize> = HashSet::from_iter(v);
    ```

## 座標回転

    - 90 度座標を回転させる

    ```rust
    fn rotate(sv: Vec<(isize, isize)>, n: usize) -> Vec<(isize, isize)> {
        let mut res = vec![];
        for i in &sv {
            res.push((n as isize - 1 - i.1, i.0));
        }
        res
    }
    ```

    - 文字列

    ```rust
    fn rotate(s: Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
        let mut res: Vec<Vec<char>> = vec![vec![]; n];
        for i in 0..n {
            for j in 0..n {
                res[i].push(s[n - 1 - j][i]);
            }
        }
        res
    }
    ```

## 進数変換

    - String a を K 進数から 10 進数に変換(Result を返すため必要に応じて unwrap() など)

    ```rust
    usize::from_str_radix(&a, k)
    ```
