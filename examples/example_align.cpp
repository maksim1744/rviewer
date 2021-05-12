#include "bits/stdc++.h"

using namespace std;

int main() {
    ios_base::sync_with_stdio(false); cin.tie(NULL); cout.tie(NULL);

    cout << "size (30,30)\n";
    // default align is CC
    cout << "grid c=(15,15) d=(3,3) s=(30,30) col=(255,255,255)\n";
    string s = "BCE";
    vector<double> v = {5, 15, 25};
    for (int i = 0; i < 3; ++i) {
        for (int j = 0; j < 3; ++j) {
            cout << "rect c=(" << v[i] << "," << v[j] << ") s=(4,3) f=0 col=(255,0,0) a=" << s[i] << s[j] << "\n";
            cout << "grid c=(" << v[i] << "," << v[j] << ") s=(3,2) d=(3,2) f=0 col=(0,255,0) a=" << s[i] << s[j] << "\n";
            cout << "text c=(" << v[i] << "," << v[j] << ") s=1 col=(0,255,255) a=" << s[i] << s[j] << " m=" << s[i] << s[j] << "\n";
            cout << "circle c=(" << v[i] << "," << v[j] << ") r=0.2 f=1 col=(255,255,255)\n";
        }
    }

    return 0;
}