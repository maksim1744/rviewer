use rviewer_client::*;

fn main() {
    let w = &mut std::io::BufWriter::new(std::fs::File::create("example_align.txt").unwrap());

    Init::new().size((30., 30.)).draw(w);
    // default align is CC
    Grid::new().center((15., 15.)).size((30., 30.)).dims((3, 3)).color(Color::WHITE).draw(w);

    let v = [5., 15., 25.];
    let aligns = [Alignment::Begin, Alignment::Center, Alignment::End];

    for i in 0..3 {
        for j in 0..3 {
            Rect::new()
                .center((v[i], v[j]))
                .size((4., 3.))
                .color(Color::RED)
                .alignment((aligns[i], aligns[j]))
                .draw(w);
            Grid::new()
                .center((v[i], v[j]))
                .size((3., 2.))
                .dims((3, 2))
                .color(Color::GREEN)
                .alignment((aligns[i], aligns[j]))
                .draw(w);
            Text::new()
                .text([aligns[i].to_char(), aligns[j].to_char()].into_iter().collect::<String>())
                .center((v[i], v[j]))
                .color(Color::CYAN)
                .alignment((aligns[i], aligns[j]))
                .draw(w);
            Circle::new().center((v[i], v[j])).radius(0.2).fill(true).color(Color::WHITE).draw(w);
        }
    }
}
