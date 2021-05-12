#include "cpp_client.h"
#include "bits/stdc++.h"

using namespace std;
using namespace rviewer;

int main() {
    ios_base::sync_with_stdio(false); cin.tie(NULL); cout.tie(NULL);

    Init().size({30, 30});
    // default align is CC
    Grid().center({15, 15}).size({30, 30}).dimensions({3, 3}).color(Color::white);
    string s = "BCE";
    vector<double> v = {5, 15, 25};
    for (int i = 0; i < 3; ++i) {
        for (int j = 0; j < 3; ++j) {
            string tmp;
            tmp += s[i];
            tmp += s[j];
            Rect({v[i], v[j]}, {4, 3}).fill(0).color(Color::red).align(s[i], s[j]);
            Grid({v[i], v[j]}, {3, 2}, {3, 2}).color(Color::green).align(s[i], s[j]);
            Text(tmp).center({v[i], v[j]}).color(Color::cyan).align(s[i], s[j]);
            Circle({v[i], v[j]}, 0.2).fill(1).color(Color::white);
        }
    }

    return 0;
}