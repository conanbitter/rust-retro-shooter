use anyhow::Result;
use crossterm::{
    cursor, execute, queue,
    style::{self, Color},
    terminal::{self, ClearType},
};
use std::{
    fmt::format,
    io::{stdout, Stdout, Write},
};

pub struct Tui {
    out: Stdout,
    width: u16,
    offset: u16,
}

impl Tui {
    pub fn new() -> Result<Tui> {
        let width = terminal::size()?.0;
        terminal::enable_raw_mode()?;
        let mut stdout = stdout();

        execute!(stdout, terminal::EnterAlternateScreen, terminal::Clear(ClearType::All))?;
        Ok(Tui {
            out: stdout,
            width,
            offset: 0,
        })
    }

    pub fn show_logo(&mut self) -> Result<()> {
        let offset = (self.width - 63) / 2;
        execute!(
            self.out,
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
        self.offset = 7;
        Ok(())
    }

    pub fn prepare_block(&mut self, caption: &str, y: u16, height: u16) -> Result<()> {
        let left_pad = (self.width as usize - 2 - caption.len() - 2) / 2;
        let right_pad = self.width as usize - 2 - caption.len() - 2 - left_pad;
        let top = format!("╔{} {} {}╗", "═".repeat(left_pad), caption, "═".repeat(right_pad));
        queue!(
            self.out,
            style::ResetColor,
            cursor::MoveTo(0, y),
            terminal::Clear(ClearType::FromCursorDown),
            style::SetBackgroundColor(Color::Grey),
            style::SetForegroundColor(Color::Black),
            cursor::MoveTo(0, y),
            style::Print(top)
        )?;

        let middle = format!("║{}║", " ".repeat(self.width as usize - 2));

        for i in 0..height {
            queue!(self.out, cursor::MoveTo(0, i + y + 1), style::Print(&middle))?;
        }

        let bottom = format!("╚{}╝", "═".repeat(self.width as usize - 2));

        queue!(self.out, cursor::MoveTo(0, y + height + 1), style::Print(bottom))?;

        self.out.flush()?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        execute!(self.out, cursor::Show, terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
