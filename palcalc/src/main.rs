use std::{
    io::{stdout, Write},
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

fn main() {
    let args = Args::parse_from(wild::args());
    /*
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

    thread::sleep(Duration::from_secs(2));

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
    */
}
