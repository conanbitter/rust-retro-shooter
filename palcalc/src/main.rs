use std::{
    io::{stdout, Stdout, Write},
    path::PathBuf,
    thread,
    time::Duration,
};

use clap::Parser;
use crossterm::{
    cursor, execute, queue,
    style::{self, Color},
    terminal::{self, ClearType},
};

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

fn pr_print(out: &mut Stdout, width: u16, x: u16, y: u16, step: u32, count: u32) {
    let percent = format!("{}%", ((step as f64) * 100.0 / (count as f64)).round() as i32);
    let left_pad = (width as usize - percent.len()) / 2;
    let right_pad = width as usize - percent.len() - left_pad;
    let line = format!("{}{}{}", " ".repeat(left_pad), percent, " ".repeat(right_pad));
    let division = (step as f64 / count as f64 * width as f64).round() as usize;
    let left_half = &line[..division];
    let right_half = &line[division..];
    queue!(
        *out,
        style::SetForegroundColor(Color::White),
        cursor::MoveTo(x, y),
        style::SetBackgroundColor(Color::DarkGreen),
        style::Print(left_half),
        cursor::MoveTo(x + division as u16, y),
        style::SetBackgroundColor(Color::DarkGrey),
        style::Print(right_half),
    )
    .unwrap();
}

fn main() {
    //let args = Args::parse_from(wild::args());

    let (mut stdout, width) = interface::prepare_terminal().unwrap();
    let offset = interface::show_logo(&mut stdout, width).unwrap();

    interface::prepare_block(&mut stdout, "Loading images", offset, 5, width).unwrap();

    let pb_width = width - 2;
    let pb_x = 1;
    let pb_y = 9;

    for i in 0..267 {
        pr_print(&mut stdout, pb_width, pb_x, pb_y, i, 268);
        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(100));
    }

    interface::finish_terminal(&mut stdout).unwrap();
}
