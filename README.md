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

## 三点が三角形をなすかどうか判定する

    - 座標 (x1,y1) , (x2,y2) , (x3,y3) があったとき以下のようになる(外積で面積が 0 でないことを示す)

    ```
    (x2−x1)(y3−y1)−(x3−x1)(y2−y1) != 0
    ```

## 0 埋め

    - n を 0 埋めにする(例では3桁)

    ```rust
    let n = 1;
    let s = format!("{0:>3}",n);
    ```

## 配列

    - 要素をスキップする(0 ではなく n から開始する)

    ```rust
    for (index,element) in v.iter().enumerate().skip(n){

    }
    ```

## 区間スケジュール

    - 区間のおおきいほうを昇順にソートしておおきいほうを基準として計算を回していく

    ```rust
    let mut lr = vec![(1,3),(2,2),(3,1)];
    lr.sort_by(|a,b|a.1.cmp(&b.1));
    let mut current = isize::MIN;
    let base = 1;
    for (l,r) in lr{
        if current + base < l{
            //何か処理
        }
    }
    ```

## 配列の結合

    - extend_from_slice

    ```rust
    let from = vec![1,2,3,4,5,6];
    let mut v = vec![];
    let n = 3;
    v.extend_from_slice(from[0..n]);
    v.extend_from_slice(from[n..5]);
    // [1,2,3,4]
    ```

## BTreeMap

- pop_first()

  - BTreeMap 内の一番小さい key を pop する
    - BTreeMap を更新することもできる

  ```rust
  let mut bmap = BTreeMap::new();
  for i in 0..n{
      bmap.insert(i,i);
  }
  while let Some((k,v)) = bmap.pop_first(){
      // 処理
  }
  ```

- pop_last()

  - BTreeMap 内の一番大きい key を pop する
    - BTreeMap を更新することもできる

  ```rust
  let mut bmap = BTreeMap::new();
  for i in 0..n{
      bmap.insert(i,i);
  }
  while let Some((k,v)) = bmap.pop_last(){
      // bmap の更新処理
      bmap.insert(k*2,v+2);
  }
  ```
