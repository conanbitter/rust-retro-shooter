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

    let mut stdout = stdout();
    let width = terminal::size().unwrap().0;
    let offset = (width - 63) / 2;

    terminal::enable_raw_mode().unwrap();
    queue!(
        stdout,
        terminal::EnterAlternateScreen,
        terminal::Clear(ClearType::All),
        style::SetForegroundColor(Color::White),
        style::SetBackgroundColor(Color::Red),
        cursor::Hide,
        cursor::MoveTo(offset, 1),
        style::Print(r"  __   ___ ___  __   __      __        __   __  ___  ___  __   "),
        cursor::MoveTo(offset, 2),
        style::Print(r" |__) |__   |  |__) /  \    /__` |__| /  \ /  \  |  |__  |__)  "),
        cursor::MoveTo(offset, 3),
        style::Print(r" |  \ |___  |  |  \ \__/    .__/ |  | \__/ \__/  |  |___ |  \  "),
        cursor::MoveTo(offset, 4),
        style::Print(r"                                                               "),
        cursor::MoveTo(offset, 5),
        style::SetBackgroundColor(Color::DarkBlue),
        style::Print(r"              P A L E T T E   C A L C U L A T O R              "),
        style::SetBackgroundColor(Color::Grey),
        style::SetForegroundColor(Color::Black),
    )
    .unwrap();

    for i in 7..11 {
        queue!(stdout, cursor::MoveTo(0, i), terminal::Clear(ClearType::CurrentLine)).unwrap();
    }

    stdout.flush().unwrap();

    let pb_width = width - 2;
    let pb_x = 1;
    let pb_y = 9;

    for i in 0..267 {
        pr_print(&mut stdout, pb_width, pb_x, pb_y, i, 268);
        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(100));
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
}
