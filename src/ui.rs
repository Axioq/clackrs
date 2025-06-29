use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write, Result};

pub struct Theme {
    pub correct: Color,
    pub incorrect: Color,
    pub upcoming: Color,
    pub caret: Color,
}

pub const DARK_THEME: Theme = Theme {
    correct: Color::White,
    incorrect: Color::Red,
    upcoming: Color::DarkGrey,
    caret: Color::Yellow,
};

pub fn init_terminal() -> Result<()> {
    terminal::enable_raw_mode()?;
    stdout().execute(terminal::EnterAlternateScreen)?;
    stdout().execute(Clear(ClearType::All))?;
    Ok(())
}

pub fn cleanup_terminal() -> Result<()> {
    terminal::disable_raw_mode()?;
    stdout().execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}

pub fn draw_prompt(text: &str) -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.execute(Print("Type the following:\n"))?;
    stdout.execute(Print(text))?;
    stdout.execute(cursor::MoveTo(0, 2))?;
    Ok(())
}

pub fn draw_word_stream(expected: &str, typed: &str) -> Result<()> {
    let mut stdout = stdout();
    let theme = DARK_THEME;

    stdout.execute(cursor::MoveTo(0, 3))?;
    stdout.execute(Clear(ClearType::FromCursorDown))?;

    for (i, (exp, act)) in expected.chars().zip(typed.chars()).enumerate() {
        if exp == act {
            stdout.execute(SetForegroundColor(theme.correct))?;
        } else {
            stdout.execute(SetForegroundColor(theme.incorrect))?;
        }
        stdout.execute(Print(exp))?;
    }

    for c in expected.chars().skip(typed.len()) {
        stdout.execute(SetForegroundColor(theme.upcoming))?;
        stdout.execute(Print(c))?;
    }

    let caret_x = typed.len() as u16;
    stdout.execute(cursor::MoveTo(caret_x, 4))?;
    stdout.execute(SetForegroundColor(theme.caret))?;
    stdout.execute(Print("^"))?;

    stdout.execute(ResetColor)?;
    stdout.flush()?;
    Ok(())
}