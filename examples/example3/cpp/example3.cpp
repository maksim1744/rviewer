#include "cpp_client.h"
#include "bits/stdc++.h"

using namespace std;
using namespace rviewer;

// visualization for https://binarysearch.com/problems/Most-Frequent-Number-in-Intervals

int main() {
    vector<pair<int, int>> intervals = {
        {1, 4},
        {3, 5},
        {6, 9},
        {7, 9}
    };

    int mn = 1e9, mx = -1e9;
    for (auto [a, b] : intervals) {
        mx = max(mx, b);
        mn = min(mn, a);
    }

    double H = 5;
    double W = mx - mn + 1;
    int fps = 30;
    Init().size({W, H}).speed(fps).svg_width(0.02);
    Line({0, H / 2}, {W, H / 2}).color(Color::white).width(1);

    for (int i = mn; i <= mx; ++i) {
        double x = i - mn + 0.5;
        Line({x, H / 2 - 0.08}, {x, H / 2 + 0.08}).color(Color::white).width(0.8);
        Text(to_string(i)).center({x, H / 2 - 0.3}).font(0.3).color(Color::white);
    }

    vector<Color> colors = {
        Color::red,
        Color::green,
        Color::orange,
        Color::yellow
    };

    for (int i = 0; i < intervals.size(); ++i) {
        auto [l, r] = intervals[i];
        double h = H / 2 + (i + 1) * 0.1;
        Line({l - mn + 0.5, h}, {r - mn + 0.5, h}).color(colors[i % colors.size()]).width(1.2);
    }

    for (int i = fps * 0.4; i <= (int)W * fps - fps * 0.4; ++i) {
        Tick();
        double x = (double)i / fps;
        Line({x, H / 2 - 1}, {x, H / 2 + 1}).color(Color::cyan).width(1.5);
        int cur = 0;
        double ix = x - 0.5 + mn;
        for (int i = 0; i < intervals.size(); ++i) {
            auto [l, r] = intervals[i];
            if (l <= ix && ix <= r) {
                ++cur;
                double h = H / 2 + (i + 1) * 0.1;
                Line({l - mn + 0.5, h}, {r - mn + 0.5, h}).color(colors[i % colors.size()]).width(2);
            }
        }
        Text((string)"count: " + to_string(cur)).center({x, H / 2 + 1.2}).font(0.2).color(Color::cyan);
    }

    return 0;
}
