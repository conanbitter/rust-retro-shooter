use std::path::PathBuf;

use clap::Parser;
use colorcalc::{ColorCalc, ColorData};
use interface::StatusImageLoading;

mod colorcalc;
mod colors;
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
    let args = Args::parse_from(wild::args());

    let mut tui = interface::Tui::new().unwrap();
    tui.show_logo().unwrap();

    let fixed_images = args.fixed_files;
    let adjustable_images = {
        let mut vector = args.files;
        let mut add_vector = args.tex_files;
        vector.append(&mut add_vector);
        vector
    };

    let total_files = (fixed_images.len() + adjustable_images.len()) as u32;

    let mut adjustable_colors = ColorData::new();
    let mut fixed_colors = ColorData::new();

    let mut status_loading = StatusImageLoading::new(&mut tui, total_files).unwrap();

    status_loading.timer.start();
    status_loading.update(&mut tui, "", 0).unwrap();
    let mut progress = 0;

    for filename in adjustable_images.iter() {
        adjustable_colors.add(&filename).unwrap();
        progress += 1;
        if status_loading.timer.needs_update() {
            status_loading
                .update(&mut tui, filename.to_str().unwrap(), progress)
                .unwrap();
        }
        //thread::sleep(Duration::from_millis(300));
    }
    for filename in fixed_images.iter() {
        fixed_colors.add(&filename).unwrap();
        progress += 1;
        if status_loading.timer.needs_update() {
            status_loading
                .update(&mut tui, filename.to_str().unwrap(), progress)
                .unwrap();
        }
        //thread::sleep(Duration::from_millis(300));
    }

    let mut calculator = ColorCalc::new(255, adjustable_colors, fixed_colors);
    let mut status_calc =
        interface::StatusCalculating::new(&mut tui, 5, 1000, calculator.unique_colors, calculator.fixed_colors)
            .unwrap();
    calculator.run(&mut status_calc, &mut tui).unwrap();

    //thread::sleep(Duration::from_secs(3));
}
