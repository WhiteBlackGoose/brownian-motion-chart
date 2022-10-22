use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};
use std::io::{stdout, Write};
use std::{thread, time};
use rand_distr::Distribution;


pub mod ring;
use ring::Ring;


fn draw_graph(ring: &Ring<f64>, height: i16, std: &mut std::io::Stdout) {
    std.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    let min_val = ring.iter().fold(f64::MAX, f64::min);
    let max_val = ring.iter().fold(f64::MIN, f64::max);
    let range = max_val - min_val;
    for (x, val) in ring.iter().enumerate() {
        let val = val - min_val;
        let y = val * (height as f64) / range;
        std
            .queue(cursor::MoveTo(x as u16, y as u16)).unwrap()
            .queue(style::PrintStyledContent("█".magenta())).unwrap();
    }
    stdout().flush().unwrap();
}

fn main() {
    let mut stdout = stdout();
    let (width, height) = {
        let (w, h) = term_size::dimensions().unwrap();
        (w as i16, h as i16)
    };
    let mut ring: Ring<f64> = Ring::new(width as usize, 0.0);
    let mut value: f64 = 0.0;
    let normal = rand_distr::Normal::new(0.0, 1.0).unwrap();
    loop {
        thread::sleep(time::Duration::from_millis(100));

        value += normal.sample(&mut rand::thread_rng());
        ring.push(value);

        draw_graph(&ring, height, &mut stdout);
    }
}
