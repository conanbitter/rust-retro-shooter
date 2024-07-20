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
    pub width: u16,
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

    pub fn refresh(&mut self) -> Result<()> {
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

pub struct ProgressBar {
    x: u16,
    y: u16,
    width: u16,
    pub total: u32,
    pub progress: u32,
}

impl ProgressBar {
    pub fn new(x: u16, y: u16, width: u16, total: u32) -> ProgressBar {
        ProgressBar {
            x,
            y,
            width,
            progress: 0,
            total,
        }
    }

    pub fn draw(&self, tui: &mut Tui) -> Result<()> {
        let percent = format!(
            "{}%",
            ((self.progress as f64) * 100.0 / (self.total as f64)).round() as i32
        );
        let left_pad = (self.width as usize - percent.len()) / 2;
        let right_pad = self.width as usize - percent.len() - left_pad;
        let line = format!("{}{}{}", " ".repeat(left_pad), percent, " ".repeat(right_pad));
        let division = (self.progress as f64 / self.total as f64 * self.width as f64).round() as usize;
        let left_half = &line[..division];
        let right_half = &line[division..];
        queue!(
            tui.out,
            style::SetForegroundColor(Color::White),
            cursor::MoveTo(self.x, self.y + tui.offset),
            style::SetBackgroundColor(Color::DarkGreen),
            style::Print(left_half),
            cursor::MoveTo(self.x + division as u16, self.y + tui.offset),
            style::SetBackgroundColor(Color::DarkGrey),
            style::Print(right_half),
        )?;
        Ok(())
    }
}

pub struct RightCounter {
    width: u16,
    x: u16,
    y: u16,
    pub value: u32,
    maxval: u32,
}

impl RightCounter {
    pub fn new(x: u16, y: u16, maxval: u32) -> RightCounter {
        let max_width = maxval.to_string().len();
        RightCounter {
            width: max_width as u16,
            x,
            y,
            value: 0,
            maxval,
        }
    }

    pub fn get_width(&self) -> u16 {
        self.width * 2 + 1
    }

    pub fn draw(&self, tui: &mut Tui) -> Result<()> {
        let strval = self.value.to_string();
        let padding = self.width as usize - strval.len();
        queue!(
            tui.out,
            cursor::MoveTo(self.x, self.y + tui.offset),
            style::SetForegroundColor(Color::Red),
            style::SetBackgroundColor(Color::Grey),
            style::Print(format!("{}{}/{}", " ".repeat(padding), strval, self.maxval))
        )?;
        Ok(())
    }
}

pub enum OverflowCut {
    Left,
    Right,
}

pub struct Label {
    width: u16,
    x: u16,
    y: u16,
    pub value: String,
    cut: OverflowCut,
}

impl Label {
    pub fn new(x: u16, y: u16, width: u16, cut: OverflowCut) -> Label {
        Label {
            width,
            x,
            y,
            value: "".to_string(),
            cut,
        }
    }

    pub fn draw(&self, tui: &mut Tui) -> Result<()> {
        queue!(
            tui.out,
            cursor::MoveTo(self.x, self.y + tui.offset),
            style::SetForegroundColor(Color::Red),
            style::SetBackgroundColor(Color::Grey)
        )?;
        if self.value.len() < self.width as usize {
            queue!(tui.out, style::Print(&self.value))?;
        } else {
            match self.cut {
                OverflowCut::Left => {
                    let start = self.value.len() - self.width as usize + 3;
                    queue!(tui.out, style::Print(format!("...{}", &self.value[start..])))?
                }
                OverflowCut::Right => {
                    let end = self.width as usize - 3;
                    queue!(tui.out, style::Print(format!("{}...", &self.value[..end])))?
                }
            }
        }
        Ok(())
    }
}
