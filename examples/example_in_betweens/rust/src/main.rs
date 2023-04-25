use rviewer_client::*;
use std::f64::consts::PI;

fn main() {
    let w = &mut std::io::BufWriter::new(std::fs::File::create("example_in_betweens.txt").unwrap());

    const W: f64 = 80.;
    const H: f64 = 60.;

    Init::new().size((W, H)).speed(30.).in_betweens(60).flipy(true).svg_width(0.08).draw(w);
    Rect::new().center((W / 2., H / 2.)).size((W, H)).color(Color::WHITE).width(1.).draw(w);
    Line::new().start((0., H / 3.)).finish((W, H / 3.)).color(Color::WHITE).width(1.).draw(w);
    Line::new()
        .start((0., H * 2. / 3.))
        .finish((W, H * 2. / 3.))
        .color(Color::WHITE)
        .width(1.)
        .draw(w);

    let f1 = (1..=59).map(|i| ((i as f64 / 60. * PI - PI / 2.).sin() + 1.) / 2.).collect::<Vec<_>>();
    let f2 = (1..=59)
        .map(|i| (((i as f64 / 60. * PI).sin() - 1.) * if i >= 30 { -1. } else { 1. } + 1.) / 2.)
        .collect::<Vec<_>>();

    set_func("f1", &f1, w);
    set_func("f2", &f2, w);

    for i in 0..3 {
        let h = H * (i * 2 + 1) as f64 / 6.;
        Line::new().start((10., h)).finish((40., h)).color(Color::WHITE).width(0.8).draw(w);
        Circle::new().center((10., h)).radius(0.4).color(Color::WHITE).fill(true).draw(w);
        Circle::new().center((40., h)).radius(0.4).color(Color::WHITE).fill(true).draw(w);
    }

    let pent = (0..5)
        .map(|i| i as f64 * 0.4 * PI - PI / 2.)
        .map(|ang| (ang.cos() * 4., ang.sin() * 4.))
        .collect::<Vec<_>>();

    let funcs = ["line", "f1", "f2"];

    let first = |i: usize, w: &mut std::io::BufWriter<std::fs::File>| {
        tick(w);
        for (lane, &func) in funcs.iter().enumerate() {
            let s = (lane * 20) as f64;
            let zero = lane as i32 * 100;

            Circle::new()
                .center((10., 10. + s))
                .radius(3.)
                .color(Color::CYAN)
                .fill(true)
                .id(zero)
                .func(func)
                .draw(w);

            let line = Line::new().color(Color::GREEN).width(1.).id(zero + 1).func(func);
            if i == 0 {
                line.start((45., 7. + s)).finish((45., 13. + s)).draw(w);
            } else {
                line.start((45., 13. + s)).finish((45., 7. + s)).draw(w);
            }

            let mut p = Poly::new().width(1.).color(Color::YELLOW).id(zero + 2).func(func);
            for j in 0..=5 {
                p = p.point((pent[(j + i) % 5].0 + 55., pent[(j + i) % 5].1 + 10. + s));
            }
            p.draw(w);

            Text::new()
                .text(String::from_utf8(vec![b'|'; 15]).unwrap())
                .center((70., 5. + s))
                .font(3.)
                .color(Color::WHITE)
                .id(zero + 3)
                .func(func)
                .draw(w);

            Rect::new()
                .center((63., 13. + s))
                .size((1., 1.))
                .fill(true)
                .color(Color::BLUE)
                .id(zero + 4)
                .func(func)
                .draw(w);
        }
    };

    let second = |w: &mut std::io::BufWriter<std::fs::File>| {
        tick(w);
        for (lane, &func) in funcs.iter().enumerate() {
            let s = (lane * 20) as f64;
            let zero = lane as i32 * 100;

            Circle::new()
                .center((40., 10. + s))
                .radius(3.)
                .color(Color::rgba(255, 165, 0, 100))
                .fill(true)
                .id(zero)
                .func(func)
                .draw(w);

            Line::new()
                .start((48., 10. + s))
                .finish((42., 10. + s))
                .color(Color::GREEN)
                .width(5.)
                .id(zero + 1)
                .func(func)
                .draw(w);

            let mut p = Poly::new().width(1.).color(Color::YELLOW).id(zero + 2).func(func);
            for i in 0..=5 {
                p = p.point((pent[i * 2 % 5].0 + 55., pent[i * 2 % 5].1 + 10. + s));
            }
            p.draw(w);

            Text::new()
                .center((70., 5. + s))
                .font(3.)
                .color(Color::WHITE)
                .id(zero + 3)
                .func(func)
                .draw(w);

            Rect::new()
                .center((77., 13. + s))
                .size((1., 9.))
                .fill(true)
                .color(Color::RED)
                .id(zero + 4)
                .func(func)
                .draw(w);
        }
    };

    first(0, w);
    second(w);
    first(1, w);
}
