use std::io::{stdout, Write};

use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

fn main() {
    let mut stdout = stdout();
    execute!(
        stdout,
        Clear(ClearType::All),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Red),
        MoveTo(0, 1),
        Print(r"  __   ___ ___  __   __      __        __   __  ___  ___  __   "),
        MoveToNextLine(1),
        Print(r" |__) |__   |  |__) /  \    /__` |__| /  \ /  \  |  |__  |__)  "),
        MoveToNextLine(1),
        Print(r" |  \ |___  |  |  \ \__/    .__/ |  | \__/ \__/  |  |___ |  \  "),
        MoveToNextLine(1),
        Print(r"                                                               "),
        MoveToNextLine(1),
        SetBackgroundColor(Color::DarkBlue),
        Print(r"                      PALETTE CALCULATOR                       "),
        ResetColor
    )
    .unwrap();
    stdout.flush().unwrap();
}
