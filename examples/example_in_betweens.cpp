#include "cpp_client.h"
#include "bits/stdc++.h"

using namespace std;
using namespace rviewer;

const double pi = acos(-1.0);

int main() {
    ios_base::sync_with_stdio(false); cin.tie(NULL);

    const int w = 80, h = 60;

    Init().size({w, h}).speed(30).in_betweens(60).flipy().svg_width(0.08);
    Rect({w/2, h/2}, {w, h}).fill(0).color(Color::white).width(1);
    Line({0, h/3}, {w, h/3}).color(Color::white).width(1);
    Line({0, h*2/3}, {w, h*2/3}).color(Color::white).width(1);

    vector<double> f1(59), f2(59);
    for (int i = 1; i <= 59; ++i) {
        f1[i - 1] = (sin(i / 60. * pi - pi / 2) + 1) / 2;
        f2[i - 1] = ((sin(i / 60. * pi) - 1) * (i >= 30 ? -1 : 1) + 1) / 2;
    }

    SetFunc("f1", f1);
    SetFunc("f2", f2);

    for (int i = 0; i < 3; ++i) {
        Line({10, h*(i*2+1)/6}, {40, h*(i*2+1)/6}).color(Color::white).width(0.8);
        Circle({10, h*(i*2+1)/6}, 0.4).color(Color::white).fill(1);
        Circle({40, h*(i*2+1)/6}, 0.4).color(Color::white).fill(1);
    }

    vector<pair<double, double>> pent;
    for (int i = 0; i < 5; ++i) {
        auto ang = -pi/2 + pi*2/5*i;
        pent.emplace_back(cos(ang) * 4, sin(ang) * 4);
    }

    vector<string> funcs = {"line", "f1", "f2"};  // line is also applied by default

    auto first = [&](int i) {
        Tick();
        for (int lane = 0; lane < 3; ++lane) {
            double s = lane * 20;

            Circle({10,10+s}, 3).color(Color::cyan).fill(1).id(lane * 100 + 0).func(funcs[lane]);
            if (i == 0) {
                Line({45,7+s}, {45,13+s}).color(Color::green).width(1).id(lane * 100 + 1).func(funcs[lane]);
            } else {
                Line({45,13+s}, {45,7+s}).color(Color::green).width(1).id(lane * 100 + 1).func(funcs[lane]);
            }
            auto p = Poly().width(1).color(Color::yellow).fill(0).id(lane * 100 + 2).func(funcs[lane]);
            for (int j = 0; j <= 5; ++j) {
                p = p.point({pent[(j + i) % 5].first + 55, pent[(j + i) % 5].second + 10 + s});
            }
            p.draw();
            Text(string(15, '|')).center({70, 5+s}).font(3).color(Color::white).id(lane * 100 + 3).func(funcs[lane]);
            Rect({63,13+s}, {1,1}).fill(1).color(Color::blue).id(lane * 100 + 4).func(funcs[lane]);
        }
    };

    auto second = [&]() {
        Tick();
        for (int lane = 0; lane < 3; ++lane) {
            double s = lane * 20;

            Circle({40,10+s}, 3).color(Color(255, 165, 0, 100)).fill(1).id(lane * 100 + 0).func(funcs[lane]);
            Line({48,10+s}, {42,10+s}).color(Color::green).width(5).id(lane * 100 + 1).func(funcs[lane]);
            auto p = Poly().width(1).color(Color::yellow).fill(0).id(lane * 100 + 2).func(funcs[lane]);
            for (int i = 0; i <= 5; ++i) {
                p = p.point({pent[i * 2 % 5].first + 55, pent[i * 2 % 5].second + 10 + s});
            }
            p.draw();
            Text("").center({70, 5+s}).font(3).color(Color::white).id(lane * 100 + 3).func(funcs[lane]);
            Rect({77,13+s}, {1,9}).fill(1).color(Color::red).id(lane * 100 + 4).func(funcs[lane]);
        }
    };

    first(0);
    second();
    first(1);

    return 0;
}
