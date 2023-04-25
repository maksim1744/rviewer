use rviewer_client::*;

// visualization for https://binarysearch.com/problems/Most-Frequent-Number-in-Intervals

fn main() {
    let w = &mut std::io::BufWriter::new(std::fs::File::create("example3.txt").unwrap());

    let intervals = vec![(1, 4), (3, 5), (6, 9), (7, 9)];

    let mn = intervals.iter().map(|p| p.0).min().unwrap();
    let mx = intervals.iter().map(|p| p.1).max().unwrap();

    const H: f64 = 5.;
    #[allow(non_snake_case)]
    let W = (mx - mn + 1) as f64;

    let fps = 30.;
    Init::new().size((W, H)).speed(fps).svg_width(0.02).draw(w);
    Line::new().start((0., H / 2.)).finish((W, H / 2.)).color(Color::WHITE).width(1).draw(w);

    for i in mn..=mx {
        let x = (i - mn) as f64 + 0.5;
        Line::new()
            .start((x, H / 2. - 0.08))
            .finish((x, H / 2. + 0.08))
            .color(Color::WHITE)
            .width(0.8)
            .draw(w);
        Text::new()
            .text(i.to_string())
            .center((x, H / 2. - 0.3))
            .font(0.3)
            .color(Color::WHITE)
            .draw(w);
    }

    let colors = [Color::RED, Color::GREEN, Color::ORANGE, Color::YELLOW];

    for (i, &(l, r)) in intervals.iter().enumerate() {
        let h = H / 2. + 0.1 * (i + 1) as f64;
        Line::new()
            .start(((l - mn) as f64 + 0.5, h))
            .finish(((r - mn) as f64 + 0.5, h))
            .color(colors[i % colors.len()])
            .width(1.2)
            .draw(w);
    }

    for i in (fps * 0.4) as usize..=(fps * (W - 0.4)) as usize {
        tick(w);
        let x = i as f64 / fps;
        Line::new()
            .start((x, H / 2. - 1.))
            .finish((x, H / 2. + 1.))
            .color(Color::CYAN)
            .width(1.5)
            .draw(w);
        let mut cur = 0;
        let ix = x - 0.5 + mn as f64;
        for (i, &(l, r)) in intervals.iter().enumerate() {
            let l = l as f64;
            let r = r as f64;
            if l <= ix && ix <= r {
                cur += 1;
                let h = H / 2. + 0.1 * (i + 1) as f64;
                Line::new()
                    .start(((l - mn as f64) + 0.5, h))
                    .finish((r - mn as f64 + 0.5, h))
                    .color(colors[i % colors.len()])
                    .width(2.)
                    .draw(w);
            }
        }
        Text::new()
            .text(format!("count: {}", cur))
            .center((x, H / 2. + 1.2))
            .font(0.2)
            .color(Color::CYAN)
            .draw(w);
    }
}
