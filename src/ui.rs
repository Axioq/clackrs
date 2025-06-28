// src/ui.rs
use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};

use std::io::stdout;
use std::io::{Write, Result};

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

pub fn draw_colored_line(expected: &str, typed: &str) -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(cursor::MoveTo(0, 3))?;
    stdout.execute(Clear(ClearType::FromCursorDown))?;

    for (exp, act) in expected.chars().zip(typed.chars()) {
        if exp == act {
            stdout.execute(SetForegroundColor(Color::White))?;
            stdout.execute(Print(exp))?;
        } else {
            stdout.execute(SetForegroundColor(Color::Red))?;
            stdout.execute(Print(exp))?;
        }
    }

    if typed.len() < expected.len() {
        for c in expected.chars().skip(typed.len()) {
            stdout.execute(SetForegroundColor(Color::DarkGrey))?;
            stdout.execute(Print(c))?;
        }
    }

    stdout.execute(ResetColor)?;
    stdout.flush()?;
    Ok(())
}