use std::{path::PathBuf, thread, time::Duration};

use clap::Parser;
use interface::OverflowCut;

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

    tui.prepare_block("Loading images", 7, 5).unwrap();

    let mut pr = interface::ProgressBar::new(2, 4, tui.width - 4, 267);
    let mut rc = interface::RightCounter::new(2, 2, 267);
    let mut lbl = interface::Label::new(10, 2, 5, OverflowCut::Right);
    lbl.value = "123456".into();

    for i in 0..267 {
        pr.progress = i;
        pr.draw(&mut tui).unwrap();
        rc.value = i;
        rc.draw(&mut tui).unwrap();
        lbl.draw(&mut tui).unwrap();
        tui.refresh().unwrap();

        thread::sleep(Duration::from_millis(100));
    }
    //thread::sleep(Duration::from_secs(3));
}
