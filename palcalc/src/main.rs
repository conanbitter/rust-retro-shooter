use std::{path::PathBuf, thread, time::Duration};

use clap::Parser;
use interface::{OverflowCut, StatusCalculating, StatusImageLoading};

mod interface;

#[derive(Parser, Debug)]
struct Args {
    #[arg(required = true)]
    files: Vec<PathBuf>,
    #[arg(short = 't', long = "tex", num_args(0..))]
    tex_files: Vec<PathBuf>,
    #[arg(short='f', long = "fixed", num_args(0..))]
    fixed_files: Vec<PathBuf>,
    #[arg(short, long)]
    shades: u32,
}

fn main() {
    //let args = Args::parse_from(wild::args());

    let mut tui = interface::Tui::new().unwrap();
    tui.show_logo().unwrap();

    let mut status_palette = StatusCalculating::new(&mut tui, 5, 500, 412343, 25).unwrap();

    status_palette.timer.start();
    status_palette.update(&mut tui, 0, 0, 0, 0.0, 0, 2500).unwrap();

    for a in 0..5 {
        for s in 0..500 {
            if status_palette.timer.needs_update() {
                status_palette
                    .update(&mut tui, a, s, 3425457, 45.05676, a * 500 + s, 2500)
                    .unwrap();
            }
            thread::sleep(Duration::from_millis(100));
        }
    }
    //thread::sleep(Duration::from_secs(3));
}
