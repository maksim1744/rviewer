#include "bits/stdc++.h"

using namespace std;

// reads input for problem https://codeforces.com/gym/102892/problem/5
// set answer here:
const int cnt = 2;

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
        cand.resize(min((int)cand.size(), cnt));
        for (auto [a, b] : cand)
            picks[i].push_back(b);
    }
    double W = 10;
    cout << "size (" << W * (m + 1) << "," << W * (n + 1) << ")\n";
    cout << "speed 1.5\n";
    for (int i = 0; i <= n; ++i) {
        cout << "line s=(" << W << "," << i * W + W << ") f=(" << W * m + W << "," << i * W + W << ") col=(255,255,255) w=0.5\n";
    }
    for (int j = 1; j <= m + 1; ++j) {
        cout << "line s=(" << W * j << "," << W << ") f=(" << W * j << "," << n * W + W << ") col=(255,255,255) w=0.5\n";
    }

    for (int i = 0; i < n; ++i) {
        cout << "text c=(" << W/2 << "," << W/2 + W + i * W << ") m=" << scores[i] << " s=" << W*0.6 << " col=(255,255,255)\n";
        for (int j = 0; j < m; ++j) {
            cout << "text c=(" << W/2 + W * (j + 1) << "," << W/2 + W + i * W << ") m=" << v[i][j] << " s=" << W*0.6 << " col=(255,255,255)\n";
        }
    }
    cout << "tick\n";
    vector<int> s(m);
    for (int i = 0; i < m; ++i) {
        cout << "tick\n";
        cout << "line s=(" << i*W + W*1.5 << "," << W*(n+1) + W*0.2 << ") f=(" << i*W + W*1.5 << "," << W*(n+1) + W - W*0.2 << ") col=(255,255,255) w=0.5\n";
        cout << "line s=(" << i*W + W*1.5 << "," << W*(n+1) + W*0.2 << ") f=(" << i*W + W*1.5 - W*0.2 << "," << W*(n+1) + W * 0.5 << ") col=(255,255,255) w=0.5\n";
        cout << "line s=(" << i*W + W*1.5 << "," << W*(n+1) + W*0.2 << ") f=(" << i*W + W*1.5 + W*0.2 << "," << W*(n+1) + W * 0.5 << ") col=(255,255,255) w=0.5\n";
        for (int j : picks[i]) {
            cout << "rect c=(" << i*W + W*1.5 << "," << W*1.5 + W*j << ") s=(" << W << "," << W << ") f=0 col=(0,255,0) w=2\n";
            cout << "text c=(" << W/2 << "," << W/2 + W + j * W << ") m=" << scores[j] << " s=" << W*0.6 << " col=(0,255,0)\n";
            s[i] += scores[j];
        }
        for (int j = 0; j < i; ++j)
            cout << "text c=(" << W/2 + W * (j + 1) << "," << W/2 << ") m=" << s[j] << " s=" << W*0.4 << " col=(255,255,255)\n";
        cout << "text c=(" << W/2 + W * (i + 1) << "," << W/2 << ") m=" << s[i] << " s=" << W*0.4 << " col=(0,255,0)\n";
    }
    cout << "tick\n";
    for (int j = 0; j < m; ++j)
        cout << "text c=(" << W/2 + W * (j + 1) << "," << W/2 << ") m=" << s[j] << " s=" << W*0.4 << " col=(255,255,255)\n";
    for (int j = 0; j + 1 < m; ++j)
        cout << "text c=(" << W + W * (j + 1) << "," << W/2 << ") m=+" << " s=" << W*0.4 << " col=(255,255,255)\n";
    cout << "text c=(" << W + W * (m) << "," << W/2 << ") m==" << " s=" << W*0.4 << " col=(255,255,255)\n";
    cout << "text c=(" << W/2 + W * (m + 1) << "," << W/2 << ") m=" << accumulate(s.begin(), s.end(), 0) << " s=" << W*0.4 << " col=(255,255,255)\n";

    return 0;
}