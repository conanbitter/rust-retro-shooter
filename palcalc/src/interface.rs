use anyhow::Result;
use crossterm::{
    cursor, execute, queue,
    style::{self, Color},
    terminal::{self, ClearType},
};
use std::io::{stdout, Stdout, Write};

pub fn prepare_terminal() -> Result<(Stdout, u16)> {
    let width = terminal::size()?.0;
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();

    execute!(stdout, terminal::EnterAlternateScreen, terminal::Clear(ClearType::All))?;
    Ok((stdout, width))
}

pub fn show_logo(out: &mut Stdout, width: u16) -> Result<u16> {
    let offset = (width - 63) / 2;
    execute!(
        out,
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
    )?;
    Ok(7)
}

pub fn prepare_block(out: &mut Stdout, caption: &str, y: u16, height: u16, width: u16) -> Result<()> {
    queue!(
        out,
        style::SetBackgroundColor(Color::Grey),
        style::SetForegroundColor(Color::Black)
    )?;

    for i in 0..height {
        queue!(out, cursor::MoveTo(0, i + y), terminal::Clear(ClearType::CurrentLine))?;
    }

    out.flush().unwrap();
    Ok(())
}

pub fn finish_terminal(out: &mut Stdout) -> Result<()> {
    execute!(out, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode().unwrap();
    Ok(())
}
