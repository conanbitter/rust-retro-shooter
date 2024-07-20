use std::{path::PathBuf, thread, time::Duration};

use clap::Parser;
use interface::{OverflowCut, StatusImageLoading};

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

    let mut status_images = StatusImageLoading::new(&mut tui, 267).unwrap();

    status_images.timer.start();
    status_images.update(&mut tui, " ", 0).unwrap();

    for i in 0..267 {
        if status_images.timer.needs_update() {
            status_images.update(&mut tui, "xdfxtghst", i).unwrap();
        }
        thread::sleep(Duration::from_millis(300));
    }
    //thread::sleep(Duration::from_secs(3));
}
