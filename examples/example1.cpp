#include "cpp_client.h"
#include "bits/stdc++.h"

using namespace std;
using namespace rviewer;

// reads input for problem https://codeforces.com/gym/102892/problem/5
// set answer here:
const int answer = 2;

int main() {
    ios_base::sync_with_stdio(false); cin.tie(NULL); cout.tie(NULL);

    int n, m, k;
    cin >> n >> m >> k;
    vector<int> scores(n);
    for (int i = 0; i < n; ++i) {
        cin >> scores[i];
    }
    vector<string> v(n);
    for (int i = 0; i < n; ++i) {
        cin >> v[i];
    }
    reverse(v.begin(), v.end());
    reverse(scores.begin(), scores.end());
    vector<vector<int>> picks(m);
    for (int i = 0; i < m; ++i) {
        vector<pair<int, int>> cand;
        for (int j = 0; j < n; ++j) {
            if (v[j][i] == '1')
                cand.emplace_back(scores[j], j);
        }
        sort(cand.begin(), cand.end());
        reverse(cand.begin(), cand.end());
        cand.resize(min((int)cand.size(), answer));
        for (auto [a, b] : cand)
            picks[i].push_back(b);
    }
    double W = 10;

    Init().size({W * (m + 2), W * (n + 2)}).speed(1.5);
    Grid({W, W}, {W*m, W*n}, {m, n}).align(Alignment::BEGIN, Alignment::BEGIN).width(0.5).color(Color::white);

    for (int i = 0; i < n; ++i) {
        // score for channel
        Text(to_string(scores[i])).center({W/2, W/2 + W + i*W}).font(W * 0.6).color(Color::white);
        for (int j = 0; j < m; ++j)
            // 0 or 1 in corresponding cell
            Text(v[i].substr(j, 1)).center({W/2 + W * (j + 1), W/2 + W + i * W}).font(W * 0.6).color(Color::white);
    }

    Tick();

    vector<int> s(m);
    for (int i = 0; i < m; ++i) {
        Tick();
        // 3 segment for an arrow
        Line().start({i*W + W*1.5, W*(n+1) + W*0.2}).finish({i*W + W*1.5, W*(n+1) + W*0.8}).color(Color::white).width(0.5);
        Line().start({i*W + W*1.5, W*(n+1) + W*0.2}).finish({i*W + W*1.5 - W*0.2, W*(n+1) + W*0.5}).color(Color::white).width(0.5);
        Line().start({i*W + W*1.5, W*(n+1) + W*0.2}).finish({i*W + W*1.5 + W*0.2, W*(n+1) + W*0.5}).color(Color::white).width(0.5);
        for (int j : picks[i]) {
            // select rectangle and corresponding channel score
            Rect({(i+1)*W, (j+1)*W}, {W, W}).align(Alignment::BEGIN, Alignment::BEGIN).fill(0).color(Color::green).width(2);
            Text(to_string(scores[j])).center({W/2, W/2 + W + j * W}).font(W * 0.6).color(Color::green);
            s[i] += scores[j];
        }
        // previous column sums
        for (int j = 0; j < i; ++j)
            Text(to_string(s[j])).center({W/2 + W*(j+1), W/2}).font(W*0.4).color(Color::white);
        // new column sum
        Text(to_string(s[i])).center({W/2 + W*(i+1), W/2}).font(W*0.4).color(Color::green);
    }

    Tick();

    // column sums
    for (int j = 0; j < m; ++j)
        Text(to_string(s[j])).center({W/2 + W*(j+1), W/2}).font(W*0.4).color(Color::white);
    // + signs
    for (int j = 0; j + 1 < m; ++j)
        Text("+").center({W + W*(j+1), W/2}).font(W*0.4).color(Color::white);
    // = sign
    Text("=").center({W + W*m, W/2}).font(W*0.4).color(Color::white);
    // total score
    Text(to_string(accumulate(s.begin(), s.end(), 0))).center({W/2 + W*(m+1), W/2}).font(W*0.4).color(Color::white);

    return 0;
}