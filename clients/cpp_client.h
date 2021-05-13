#include <iostream>
#include <optional>
#include <utility>
#include <vector>
#include <string>

namespace rviewer {

// you can change to auto& _cout = std::cerr;
auto& _cout = std::cout;
using std::optional, std::pair, std::vector, std::string, std::make_pair;

struct Color {
    uint8_t r = 0, g = 0, b = 0;
    uint8_t alpha = 255;

    Color(uint8_t r, uint8_t g, uint8_t b, uint8_t alpha = 255) : r(r), g(g), b(b), alpha(alpha) {}
    Color(uint8_t gray = 0) : r(gray), g(gray), b(gray), alpha(255) {}

    static const Color black;
    static const Color white;
    static const Color blue;
    static const Color green;
    static const Color red;
    static const Color yellow;
    static const Color cyan;
    static const Color magenta;
    static const Color orange;
};

const Color Color::black = Color(0, 0, 0);
const Color Color::white = Color(255, 255, 255);
const Color Color::red = Color(255, 0, 0);
const Color Color::green = Color(0, 255, 0);
const Color Color::blue = Color(0, 0, 255);
const Color Color::yellow = Color(255, 255, 0);
const Color Color::cyan = Color(0, 255, 255);
const Color Color::magenta = Color(255, 0, 255);
const Color Color::orange = Color(255, 165, 0);

enum Alignment {
    BEGIN,
    CENTER,
    END,
};

string rviewer_to_string(double d) {
    return std::to_string(d);
}
string rviewer_to_string(int i) {
    return std::to_string(i);
}
template<typename T>
string rviewer_to_string(const pair<T, T> &p) {
    return "(" + rviewer_to_string(p.first) + "," + rviewer_to_string(p.second) + ")";
}
string rviewer_to_string(const Color &c) {
    string result = "(" + std::to_string(c.r) + "," + std::to_string(c.g) + "," + std::to_string(c.b);
    if (c.alpha != 255)
        result += "," + std::to_string(c.alpha);
    result += ")";
    return result;
}
string rviewer_to_string(const Alignment &alignment) {
    if (alignment == Alignment::BEGIN) return "B";
    if (alignment == Alignment::CENTER) return "C";
    if (alignment == Alignment::END) return "E";
    return "";
}
string rviewer_to_string(const pair<Alignment, Alignment> &alignment) {
    return rviewer_to_string(alignment.first) + rviewer_to_string(alignment.second);
}
string rviewer_to_string(const string &s) {
    return s;
}
template<typename T>
void print_option(const string &name, const optional<T> &o) {
    if (o)
        _cout << ' ' << name << '=' << rviewer_to_string(*o);
}

void Tick() {
    _cout << "tick\n";
    _cout.flush();
}

void DisableTag(const string &s) {
    _cout << "disable " << s << '\n';
}

struct Init {
    optional<pair<double, double>> size_;
    optional<double> font_;
    optional<double> speed_;
    optional<double> width_;
    optional<double> svg_width_;
    bool drawn_ = false;

    Init() {}
    Init &operator = (const Init &other) = default;
    Init(Init &r) {
        *this = r;
        r.drawn_ = true;
    }

    Init &size(const pair<double, double> &s) {
        size_ = s;
        return *this;
    }
    Init &font(double d) {
        font_ = d;
        return *this;
    }
    Init &speed(double d) {
        speed_ = d;
        return *this;
    }
    Init &width(double d) {
        width_ = d;
        return *this;
    }
    Init &svg_width(double d) {
        svg_width_ = d;
        return *this;
    }

    void draw() {
        drawn_ = true;
        if (size_)
            _cout << "size " << rviewer_to_string(*size_) << '\n';
        if (width_)
            _cout << "width " << rviewer_to_string(*width_) << '\n';
        if (svg_width_)
            _cout << "svgwidth " << rviewer_to_string(*svg_width_) << '\n';
        if (font_)
            _cout << "font " << rviewer_to_string(*font_) << '\n';
        if (speed_)
            _cout << "speed " << rviewer_to_string(*speed_) << '\n';
    }

    ~Init() {
        if (!drawn_) {
            draw();
        }
    }
};

struct Rect {
    optional<pair<double, double>> center_;
    optional<pair<double, double>> size_;
    optional<double> width_;
    optional<bool> fill_;
    optional<Color> color_;
    optional<pair<Alignment, Alignment>> alignment_;
    vector<string> tags_;
    bool drawn_ = false;

    Rect() {}
    Rect &operator = (const Rect &other) = default;
    Rect(Rect &r) {
        *this = r;
        r.drawn_ = true;
    }
    Rect(const pair<double, double> &center, const pair<double, double> &size) : center_(center), size_(size) {
    }
    Rect &center(const pair<double, double> &c) {
        center_ = c;
        return *this;
    }
    Rect &size(const pair<double, double> &s) {
        size_ = s;
        return *this;
    }
    Rect &width(double w) {
        width_ = w;
        return *this;
    }
    Rect &fill(bool b) {
        fill_ = b;
        return *this;
    }
    Rect &color(const Color &c) {
        color_ = c;
        return *this;
    }
    Rect &align(Alignment horizontal, Alignment vertical) {
        alignment_ = {horizontal, vertical};
        return *this;
    }
    Rect &align(char horizontal, char vertical) {
        alignment_ = make_pair(
            horizontal == 'B' ? Alignment::BEGIN : (horizontal == 'E' ? Alignment::END : Alignment::CENTER),
            vertical   == 'B' ? Alignment::BEGIN : (vertical   == 'E' ? Alignment::END : Alignment::CENTER));
        return *this;
    }
    Rect &tag(const string &s) {
        tags_.push_back(s);
        return *this;
    }

    void draw() {
        drawn_ = true;
        _cout << "rect";
        print_option("c", center_);
        print_option("s", size_);
        print_option("w", width_);
        print_option("f", fill_);
        print_option("col", color_);
        print_option("a", alignment_);
        for (const string &tag : tags_)
            _cout << " t=" << tag;
        _cout << '\n';
    }

    ~Rect() {
        if (!drawn_) {
            draw();
        }
    }
};

struct Circle {
    optional<pair<double, double>> center_;
    optional<double> radius_;
    optional<double> width_;
    optional<bool> fill_;
    optional<Color> color_;
    vector<string> tags_;
    bool drawn_ = false;

    Circle() {}
    Circle &operator = (const Circle &other) = default;
    Circle(Circle &c) {
        *this = c;
        c.drawn_ = true;
    }
    Circle(const pair<double, double> &center, double radius) : center_(center), radius_(radius) {}

    Circle &center(const pair<double, double> &c) {
        center_ = c;
        return *this;
    }
    Circle &radius(double r) {
        radius_ = r;
        return *this;
    }
    Circle &width(double w) {
        width_ = w;
        return *this;
    }
    Circle &fill(bool b) {
        fill_ = b;
        return *this;
    }
    Circle &color(const Color &c) {
        color_ = c;
        return *this;
    }
    Circle &tag(const string &s) {
        tags_.push_back(s);
        return *this;
    }

    void draw() {
        drawn_ = true;
        _cout << "circle";
        print_option("c", center_);
        print_option("r", radius_);
        print_option("w", width_);
        print_option("f", fill_);
        print_option("col", color_);
        for (const string &tag : tags_)
            _cout << " t=" << tag;
        _cout << '\n';
    }

    ~Circle() {
        if (!drawn_) {
            draw();
        }
    }
};

struct Line {
    optional<pair<double, double>> start_;
    optional<pair<double, double>> finish_;
    optional<double> width_;
    optional<Color> color_;
    vector<string> tags_;
    bool drawn_ = false;

    Line() {}
    Line &operator = (const Line &other) = default;
    Line(Line &c) {
        *this = c;
        c.drawn_ = true;
    }
    Line(const pair<double, double> &start, const pair<double, double> &finish) : start_(start), finish_(finish) {}

    Line &start(const pair<double, double> &p) {
        start_ = p;
        return *this;
    }
    Line &finish(const pair<double, double> &p) {
        finish_ = p;
        return *this;
    }
    Line &width(double w) {
        width_ = w;
        return *this;
    }
    Line &color(const Color &c) {
        color_ = c;
        return *this;
    }
    Line &tag(const string &s) {
        tags_.push_back(s);
        return *this;
    }

    void draw() {
        drawn_ = true;
        _cout << "line";
        print_option("s", start_);
        print_option("f", finish_);
        print_option("w", width_);
        print_option("col", color_);
        for (const string &tag : tags_)
            _cout << " t=" << tag;
        _cout << '\n';
    }

    ~Line() {
        if (!drawn_) {
            draw();
        }
    }
};

struct Grid {
    optional<pair<double, double>> center_;
    optional<pair<double, double>> size_;
    optional<pair<int, int>> dimensions_;
    optional<double> width_;
    optional<bool> fill_;
    optional<Color> color_;
    optional<pair<Alignment, Alignment>> alignment_;
    vector<string> tags_;
    bool drawn_ = false;

    Grid() {}
    Grid &operator = (const Grid &other) = default;
    Grid(Grid &r) {
        *this = r;
        r.drawn_ = true;
    }
    Grid(const pair<double, double> &center, const pair<double, double> &size, const pair<int, int> &dimensions) : center_(center), size_(size), dimensions_(dimensions) {
    }

    Grid &center(const pair<double, double> &c) {
        center_ = c;
        return *this;
    }
    Grid &size(const pair<double, double> &s) {
        size_ = s;
        return *this;
    }
    Grid &dimensions(const pair<int, int> &d) {
        dimensions_ = d;
        return *this;
    }
    Grid &width(double w) {
        width_ = w;
        return *this;
    }
    Grid &fill(bool b) {
        fill_ = b;
        return *this;
    }
    Grid &color(const Color &c) {
        color_ = c;
        return *this;
    }
    Grid &align(Alignment horizontal, Alignment vertical) {
        alignment_ = {horizontal, vertical};
        return *this;
    }
    Grid &align(char horizontal, char vertical) {
        alignment_ = make_pair(
            horizontal == 'B' ? Alignment::BEGIN : (horizontal == 'E' ? Alignment::END : Alignment::CENTER),
            vertical   == 'B' ? Alignment::BEGIN : (vertical   == 'E' ? Alignment::END : Alignment::CENTER));
        return *this;
    }
    Grid &tag(const string &s) {
        tags_.push_back(s);
        return *this;
    }

    void draw() {
        drawn_ = true;
        _cout << "grid";
        print_option("c", center_);
        print_option("s", size_);
        print_option("d", dimensions_);
        print_option("w", width_);
        print_option("f", fill_);
        print_option("col", color_);
        print_option("a", alignment_);
        for (const string &tag : tags_)
            _cout << " t=" << tag;
        _cout << '\n';
    }

    ~Grid() {
        if (!drawn_) {
            draw();
        }
    }
};

struct Poly {
    vector<pair<double, double>> vertices_;
    optional<double> width_;
    optional<bool> fill_;
    optional<Color> color_;
    optional<pair<Alignment, Alignment>> alignment_;
    vector<string> tags_;
    bool drawn_ = false;

    Poly() {}
    Poly &operator = (const Poly &other) = default;
    Poly(Poly &r) {
        *this = r;
        r.drawn_ = true;
    }

    Poly &point(const pair<double, double> &p) {
        vertices_.push_back(p);
        return *this;
    }
    Poly &width(double w) {
        width_ = w;
        return *this;
    }
    Poly &fill(bool b) {
        fill_ = b;
        return *this;
    }
    Poly &color(const Color &c) {
        color_ = c;
        return *this;
    }
    Poly &align(Alignment horizontal, Alignment vertical) {
        alignment_ = {horizontal, vertical};
        return *this;
    }
    Poly &align(char horizontal, char vertical) {
        alignment_ = make_pair(
            horizontal == 'B' ? Alignment::BEGIN : (horizontal == 'E' ? Alignment::END : Alignment::CENTER),
            vertical   == 'B' ? Alignment::BEGIN : (vertical   == 'E' ? Alignment::END : Alignment::CENTER));
        return *this;
    }
    Poly &tag(const string &s) {
        tags_.push_back(s);
        return *this;
    }

    void draw() {
        drawn_ = true;
        _cout << "poly";
        for (const auto &p : vertices_)
            print_option("p", optional<pair<double, double>>(p));
        print_option("w", width_);
        print_option("f", fill_);
        print_option("col", color_);
        print_option("a", alignment_);
        for (const string &tag : tags_)
            _cout << " t=" << tag;
        _cout << '\n';
    }

    ~Poly() {
        if (!drawn_) {
            draw();
        }
    }
};

struct Text {
    optional<string> text_;
    optional<pair<double, double>> center_;
    optional<double> font_;
    optional<Color> color_;
    optional<pair<Alignment, Alignment>> alignment_;
    vector<string> tags_;
    bool drawn_ = false;

    Text() {}
    Text &operator = (const Text &other) = default;
    Text(Text &r) {
        *this = r;
        r.drawn_ = true;
    }
    Text(const string &s) : text_(s) {}

    Text &text(const string &s) {
        text_ = s;
        return *this;
    }
    Text &center(const pair<double, double> &c) {
        center_ = c;
        return *this;
    }
    Text &font(double s) {
        font_ = s;
        return *this;
    }
    Text &color(const Color &c) {
        color_ = c;
        return *this;
    }
    Text &align(Alignment horizontal, Alignment vertical) {
        alignment_ = {horizontal, vertical};
        return *this;
    }
    Text &align(char horizontal, char vertical) {
        alignment_ = make_pair(
            horizontal == 'B' ? Alignment::BEGIN : (horizontal == 'E' ? Alignment::END : Alignment::CENTER),
            vertical   == 'B' ? Alignment::BEGIN : (vertical   == 'E' ? Alignment::END : Alignment::CENTER));
        return *this;
    }
    Text &tag(const string &s) {
        tags_.push_back(s);
        return *this;
    }

    void draw() {
        drawn_ = true;
        _cout << "text";
        print_option("m", text_);
        print_option("c", center_);
        print_option("s", font_);
        print_option("col", color_);
        print_option("a", alignment_);
        for (const string &tag : tags_)
            _cout << " t=" << tag;
        _cout << '\n';
    }

    ~Text() {
        if (!drawn_) {
            draw();
        }
    }
};

}  // rviewer
