use anyhow::Result;
use crossterm::{
    cursor, execute, queue,
    style::{self, Color},
    terminal::{self, ClearType},
};
use std::{
    io::{stdout, Stdout, Write},
    time::{Duration, Instant},
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
        let strval = (self.value + 1).to_string();
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
            let padding = self.width as usize - self.value.len();
            queue!(tui.out, style::Print(format!("{}{}", &self.value, " ".repeat(padding))))?;
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

pub struct Timer {
    step: u32,
    total: u32,
    start: Instant,
    last_update: Instant,
}

fn duration_format(duration: Duration) -> String {
    let hours = duration.as_secs() / 60 / 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let seconds = duration.as_secs() % 60;
    if hours > 0 {
        format!("{:>2}h {:0>2}m {:0>2}s", hours, minutes, seconds)
    } else {
        if minutes > 0 {
            format!("{:0>2}m {:0>2}s    ", minutes, seconds) //"    {:0>2}m {:0>2}s"
        } else {
            format!("{:0>2}s        ", seconds) //"        {:0>2}s"
        }
    }
}

impl Timer {
    pub fn new(total: u32) -> Timer {
        Timer {
            step: 0,
            total,
            start: Instant::now(),
            last_update: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.start = Instant::now();
        self.last_update = Instant::now();
    }

    pub fn needs_update(&self) -> bool {
        self.last_update.elapsed() > Duration::from_millis(500)
    }

    pub fn update(&mut self, step: u32) {
        self.step = step;
        self.last_update = Instant::now();
    }

    pub fn get_elapsed(&self) -> String {
        duration_format(self.start.elapsed())
    }

    pub fn get_remaining(&self) -> String {
        if self.step == 0 {
            return duration_format(Duration::ZERO);
        }
        let t = self.total as f64;
        let s = self.step as f64;
        //let total_time = t / s;
        let rem_time = (t - s) / s;
        let remaining = self.start.elapsed().mul_f64(rem_time);
        duration_format(remaining)
    }
}

pub struct StatusImageLoading {
    l_filename: Label,
    l_time_elapsed: Label,
    l_time_remaining: Label,
    c_counter: RightCounter,
    pbar: ProgressBar,
    pub timer: Timer,
}

impl StatusImageLoading {
    pub fn new(tui: &mut Tui, total_files: u32) -> Result<StatusImageLoading> {
        tui.prepare_block("Loading images", tui.offset, 6)?;
        let mut counter = RightCounter::new(0, 4, total_files);
        counter.x = tui.width - counter.get_width() - 2;
        execute!(
            tui.out,
            style::SetBackgroundColor(Color::Grey),
            style::SetForegroundColor(Color::Black),
            cursor::MoveTo(2, 2 + tui.offset),
            style::Print("Elapsed:"),
            cursor::MoveTo(tui.width / 2, 2 + tui.offset),
            style::Print("Remaining:")
        )?;
        Ok(StatusImageLoading {
            l_filename: Label::new(2, 4, tui.width - 4 - 2 - counter.get_width(), OverflowCut::Right),
            l_time_elapsed: Label::new(11, 2, 13, OverflowCut::Right),
            l_time_remaining: Label::new(tui.width / 2 + 11, 2, 13, OverflowCut::Right),
            c_counter: counter,
            pbar: ProgressBar::new(2, 5, tui.width - 4, total_files),
            timer: Timer::new(total_files),
        })
    }

    pub fn update(&mut self, tui: &mut Tui, filename: &str, progress: u32) -> Result<()> {
        self.l_filename.value = filename.into();
        self.c_counter.value = progress;
        self.pbar.progress = progress;
        self.timer.update(progress);
        self.l_time_elapsed.value = self.timer.get_elapsed();
        self.l_time_remaining.value = self.timer.get_remaining();

        self.l_filename.draw(tui)?;
        self.l_time_elapsed.draw(tui)?;
        self.l_time_remaining.draw(tui)?;
        self.c_counter.draw(tui)?;
        self.pbar.draw(tui)?;
        tui.refresh()?;
        Ok(())
    }
}

pub struct StatusCalculating {
    l_time_elapsed: Label,
    l_time_remaining: Label,
    c_attempts: RightCounter,
    c_steps: RightCounter,
    l_moved: Label,
    l_distance: Label,
    pbar: ProgressBar,
    pub timer: Timer,
}

impl StatusCalculating {
    pub fn new(
        tui: &mut Tui,
        total_attempts: u32,
        total_steps: u32,
        unique_colors: u64,
        fixed_colors: u64,
    ) -> Result<StatusCalculating> {
        tui.prepare_block("Calculating palette", tui.offset, 12)?;
        let second_column = tui.width / 2;
        execute!(
            tui.out,
            style::SetBackgroundColor(Color::Grey),
            style::SetForegroundColor(Color::Black),
            cursor::MoveTo(2, 2 + tui.offset),
            style::Print("Elapsed:"),
            cursor::MoveTo(second_column, 2 + tui.offset),
            style::Print("Remaining:"),
            cursor::MoveTo(2, 4 + tui.offset),
            style::Print("Attempt:"),
            cursor::MoveTo(second_column, 4 + tui.offset),
            style::Print("Step:"),
            cursor::MoveTo(2, 6 + tui.offset),
            style::Print("Points moved:"),
            cursor::MoveTo(second_column, 6 + tui.offset),
            style::Print("Distance:"),
            cursor::MoveTo(2, 10 + tui.offset),
            style::Print("Colors"),
            cursor::MoveTo(2, 11 + tui.offset),
            style::Print("Adjustable:"),
            cursor::MoveTo(second_column, 11 + tui.offset),
            style::Print("Fixed:"),
            style::SetForegroundColor(Color::Red),
            cursor::MoveTo(14, 11 + tui.offset),
            style::Print(unique_colors.to_string()),
            cursor::MoveTo(second_column + 7, 11 + tui.offset),
            style::Print(fixed_colors.to_string()),
        )?;
        Ok(StatusCalculating {
            l_time_elapsed: Label::new(11, 2, 13, OverflowCut::Right),
            l_time_remaining: Label::new(second_column + 11, 2, 13, OverflowCut::Right),
            c_attempts: RightCounter::new(2 + 9, 4, total_attempts),
            c_steps: RightCounter::new(second_column + 6, 4, total_steps),
            l_moved: Label::new(2 + 14, 6, second_column - 2 - 14 - 2, OverflowCut::Left),
            l_distance: Label::new(second_column + 10, 6, 14, OverflowCut::Right),
            pbar: ProgressBar::new(2, 8, tui.width - 4, total_attempts * total_steps),
            timer: Timer::new(total_attempts * total_steps),
        })
    }

    pub fn update(
        &mut self,
        tui: &mut Tui,
        attempt: u32,
        step: u32,
        moved: u64,
        distance: f64,
        progress: u32,
        adjusted_total: u32,
    ) -> Result<()> {
        self.timer.total = adjusted_total;
        self.timer.update(progress);
        self.l_time_elapsed.value = self.timer.get_elapsed();
        self.l_time_remaining.value = self.timer.get_remaining();
        self.c_attempts.value = attempt;
        self.c_steps.value = step;
        self.l_moved.value = moved.to_string();
        self.l_distance.value = format!("{:8.4}", distance);
        self.pbar.total = adjusted_total;
        self.pbar.progress = progress;

        self.l_time_elapsed.draw(tui)?;
        self.l_time_remaining.draw(tui)?;
        self.c_attempts.draw(tui)?;
        self.c_steps.draw(tui)?;
        self.l_moved.draw(tui)?;
        self.l_distance.draw(tui)?;
        self.pbar.draw(tui)?;
        tui.refresh()?;
        Ok(())
    }
}
