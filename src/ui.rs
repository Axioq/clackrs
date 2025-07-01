use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write, Result};

fn wrap_text(text: &str, width: u16) -> Vec<String> {
    let mut lines = vec![];
    let mut line = String::new();
    for word in text.split_whitespace() {
        if line.len() + word.len() + 1 > width as usize {
            lines.push(line.trim_end().to_string());
            line = String::new();
        }
        line.push_str(word);
        line.push(' ');
    }
    if !line.is_empty() {
        lines.push(line.trim_end().to_string());
    }
    lines
}

pub struct Theme {
    pub correct: Color,
    pub incorrect: Color,
    pub upcoming: Color,
    pub caret: Color,
}

pub const DARK_THEME: Theme = Theme {
    correct: Color::Green,
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
    stdout.execute(cursor::MoveTo(0, 1))?;
    stdout.execute(SetForegroundColor(DARK_THEME.upcoming))?;
    stdout.execute(Print("Type the following:\n"))?;

    let lines = wrap_text(text, terminal::size()?.0);
    for (i, line) in lines.iter().enumerate().take(2) {
        stdout.execute(cursor::MoveTo(0, (2 + i) as u16))?;
        stdout.execute(Print(line))?;
    }

    stdout.execute(ResetColor)?;
    Ok(())
}

pub fn draw_word_stream(expected: &str, typed: &str, time_left: u64) -> Result<()> {
    let mut stdout = stdout();
    let theme = DARK_THEME;

    let (term_width, _) = terminal::size()?;

    stdout.execute(cursor::MoveTo(term_width.saturating_sub(10), 0))?;
    stdout.execute(SetForegroundColor(Color::Cyan))?;
    stdout.execute(Print(format!("Time: {:>3}s", time_left)))?;
    stdout.execute(ResetColor)?;

    stdout.execute(cursor::MoveTo(0, 4))?;
    stdout.execute(SetForegroundColor(Color::DarkGrey))?;
    stdout.execute(Print("─".repeat(term_width as usize)))?;
    stdout.execute(ResetColor)?;

    stdout.execute(cursor::MoveTo(0, 5))?;
    stdout.execute(Clear(ClearType::FromCursorDown))?;

    let expected_lines = wrap_text(expected, term_width);
    let typed_lines = wrap_text(typed, term_width);

    for (y, (exp_line, typed_line)) in expected_lines.iter().zip(typed_lines.iter()).enumerate() {
        stdout.execute(cursor::MoveTo(0, (5 + y) as u16))?;
        for (exp, act) in exp_line.chars().zip(typed_line.chars()) {
            if exp == act {
                stdout.execute(SetForegroundColor(theme.correct))?;
            } else {
                stdout.execute(SetForegroundColor(theme.incorrect))?;
            }
            stdout.execute(Print(exp))?;
        }
        for c in exp_line.chars().skip(typed_line.len()) {
            stdout.execute(SetForegroundColor(theme.upcoming))?;
            stdout.execute(Print(c))?;
        }
    }

    let caret_x = typed.len() as u16 % term_width;
    let caret_y = 5 + (typed.len() as u16 / term_width);
    stdout.execute(cursor::MoveTo(caret_x, caret_y + 1))?;
    stdout.execute(SetForegroundColor(theme.caret))?;
    stdout.execute(Print("^"))?;

    stdout.execute(ResetColor)?;
    stdout.flush()?;
    Ok(())
}

use crossterm::event::{self, Event, KeyCode};

pub enum GameMode {
    SinglePlayer,
    Multiplayer,
    Exit,
}

pub fn show_menu() -> Result<GameMode> {
    let mut stdout = stdout();
    let options = ["Single Player", "Multiplayer (coming soon)", "Exit"];
    let mut selected = 0;

    terminal::enable_raw_mode()?;
    loop {
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;
        stdout.execute(SetForegroundColor(Color::White))?;
        stdout.execute(Print("Select Game Mode:\n\n"))?;

        for (i, opt) in options.iter().enumerate() {
            if i == selected {
                stdout.execute(SetForegroundColor(Color::Cyan))?;
                stdout.execute(Print(format!("> {}\n", opt)))?;
            } else {
                stdout.execute(SetForegroundColor(Color::DarkGrey))?;
                stdout.execute(Print(format!("  {}\n", opt)))?;
            }
        }

        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < options.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    return Ok(match selected {
                        0 => GameMode::SinglePlayer,
                        1 => GameMode::Multiplayer,
                        _ => GameMode::Exit,
                    });
                }
                KeyCode::Esc => return Ok(GameMode::Exit),
                _ => {}
            }
        }
    }
}

pub fn wait_for_enter() -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.execute(SetForegroundColor(Color::White))?;
    stdout.execute(Print("Press Enter to start..."))?;
    stdout.execute(ResetColor)?;
    stdout.flush()?;

    loop {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Enter {
                break;
            }
        }
    }
    Ok(())
}