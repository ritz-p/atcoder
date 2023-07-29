use proconio::input;

fn main(){
    input!{
        n: usize,
        t: usize,
        m: usize,
        ab: [(usize,usize);m]
    };
    
}
unsigned N, T, M;
vector<unsigned> hate, teams;

// 再帰関数でチーム分け


int main() {
    using namespace std;
    cin >> N >> T >> M;

    // hate[i] の j ビットめが 1 ⟹ i 番目の選手と j 番目の選手の相性が悪い (0-indexed)
    hate = vector<unsigned>(N);
    for (unsigned i{}, a, b; i < M; ++i) {
        cin >> a >> b;
        hate[--b] |= 1U << --a;
    }

    teams.reserve(T);
    cout << dfs(0) << endl;

    return 0;
}

unsigned dfs(unsigned now) {
    // 最後の選手まで見て T チームに分かれていれば OK
    if (now == N)
        return size(teams) == T;

    unsigned ans{};

    // すでにあるチームに now 番目の選手を追加する
    for (auto &&team : teams)
        // チームに now 番目の選手と相性の悪い選手がいなければ
        if (!(team & hate[now])) {
            team ^= 1U << now;
            ans += dfs(now + 1);
            team ^= 1U << now;
        }

    // チーム数に余裕があるとき、新しいチームを作る
    if (size(teams) < T) {
        teams.emplace_back(1U << now);
        ans += dfs(now + 1);
        teams.pop_back();
    }

    return ans;
}