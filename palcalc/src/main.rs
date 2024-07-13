use std::io::{stdout, Write};

use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

fn main() {
    let mut stdout = stdout();
    let offset = (terminal::size().unwrap().0 - 63) / 2;
    execute!(
        stdout,
        Clear(ClearType::All),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Red),
        MoveTo(offset, 1),
        Print(r"  __   ___ ___  __   __      __        __   __  ___  ___  __   "),
        MoveTo(offset, 2),
        Print(r" |__) |__   |  |__) /  \    /__` |__| /  \ /  \  |  |__  |__)  "),
        MoveTo(offset, 3),
        Print(r" |  \ |___  |  |  \ \__/    .__/ |  | \__/ \__/  |  |___ |  \  "),
        MoveTo(offset, 4),
        Print(r"                                                               "),
        MoveTo(offset, 5),
        SetBackgroundColor(Color::DarkBlue),
        Print(r"              P A L E T T E   C A L C U L A T O R              "),
        ResetColor
    )
    .unwrap();
    stdout.flush().unwrap();
}
