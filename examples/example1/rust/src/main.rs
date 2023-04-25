use rviewer_client::*;

fn main() {
    let w = &mut std::io::BufWriter::new(std::fs::File::create("example1.txt").unwrap());

    // hardcoded input for problem https://codeforces.com/gym/102892/problem/5
    let answer = 2;
    let n = 4;
    let m = 15;
    let scores = vec![5, 9, 3, 11];
    let v = ["000111011000000", "111111010001111", "101100011011000", "000110000001000"];
    let v = v
        .into_iter()
        .rev()
        .map(|s| s.chars().map(|c| c == '1').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut picks = Vec::new();

    for i in 0..m {
        let mut cand = Vec::new();
        for j in 0..n {
            if v[j][i] {
                cand.push((scores[j], j));
            }
        }
        cand.sort();
        cand.reverse();
        cand.truncate(answer);
        picks.push(cand.into_iter().map(|(_a, b)| b).collect::<Vec<_>>());
    }

    const W: f64 = 10.;

    Init::new().size((W * (m + 2) as f64, W * (n + 2) as f64)).speed(1.5).draw(w);
    Grid::new()
        .center((W, W))
        .size((W * m as f64, W * n as f64))
        .dims((m, n))
        .alignment((Alignment::Begin, Alignment::Begin))
        .width(0.5)
        .color(Color::WHITE)
        .draw(w);

    for i in 0..n {
        // score for channel
        Text::new()
            .text(scores[i].to_string())
            .center((W / 2., W / 2. + W + W * i as f64))
            .font(W * 0.6)
            .color(Color::WHITE)
            .draw(w);
        for j in 0..m {
            // 0 or 1 in corresponding cell
            Text::new()
                .text((v[i][j] as i32).to_string())
                .center((W / 2. + W * (j + 1) as f64, W / 2. + W + W * i as f64))
                .font(W * 0.6)
                .color(Color::WHITE)
                .draw(w);
        }
    }

    tick(w);

    let mut s = vec![0; m];
    for i in 0..m {
        tick(w);
        // 3 segment for an arrow
        Line::new()
            .start((W * i as f64 + W * 1.5, W * (n + 1) as f64 + W * 0.2))
            .finish((W * i as f64 + W * 1.5, W * (n + 1) as f64 + W * 0.8))
            .color(Color::WHITE)
            .width(0.5)
            .draw(w);
        Line::new()
            .start((W * i as f64 + W * 1.5, W * (n + 1) as f64 + W * 0.2))
            .finish((W * i as f64 + W * 1.5 - W * 0.2, W * (n + 1) as f64 + W * 0.5))
            .color(Color::WHITE)
            .width(0.5)
            .draw(w);
        Line::new()
            .start((W * i as f64 + W * 1.5, W * (n + 1) as f64 + W * 0.2))
            .finish((W * i as f64 + W * 1.5 + W * 0.2, W * (n + 1) as f64 + W * 0.5))
            .color(Color::WHITE)
            .width(0.5)
            .draw(w);

        for &j in picks[i].iter() {
            // select rectangle and corresponding channel score
            Rect::new()
                .center((W * (i + 1) as f64, W * (j + 1) as f64))
                .size((W, W))
                .alignment((Alignment::Begin, Alignment::Begin))
                .fill(false)
                .color(Color::GREEN)
                .width(2)
                .draw(w);
            Text::new()
                .text(scores[j].to_string())
                .center((W / 2., W / 2. + W + W * j as f64))
                .font(W * 0.6)
                .color(Color::GREEN)
                .draw(w);
            s[i] += scores[j];
        }

        // previous column sums
        for (j, s) in s.iter().enumerate().take(i) {
            Text::new()
                .text(s.to_string())
                .center((W / 2. + W * (j + 1) as f64, W / 2.))
                .font(W * 0.4)
                .color(Color::WHITE)
                .draw(w);
        }
        // new column sum
        Text::new()
            .text(s[i].to_string())
            .center((W / 2. + W * (i + 1) as f64, W / 2.))
            .font(W * 0.4)
            .color(Color::GREEN)
            .draw(w);
    }

    tick(w);

    // column sums
    for (j, s) in s.iter().enumerate() {
        Text::new()
            .text(s.to_string())
            .center((W / 2. + W * (j + 1) as f64, W / 2.))
            .font(W * 0.4)
            .color(Color::WHITE)
            .draw(w);
    }
    // + signs
    for j in 0..m - 1 {
        Text::new()
            .text("+")
            .center((W + W * (j + 1) as f64, W / 2.))
            .font(W * 0.4)
            .color(Color::WHITE)
            .draw(w);
    }
    // = sign
    Text::new()
        .text("=")
        .center((W + W * m as f64, W / 2.))
        .font(W * 0.4)
        .color(Color::WHITE)
        .draw(w);
    // total score
    Text::new()
        .text(s.into_iter().sum::<i32>().to_string())
        .center((W / 2. + W * (m + 1) as f64, W / 2.))
        .font(W * 0.4)
        .color(Color::WHITE)
        .draw(w);
}
