use std::io::{self, Write};
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyCode};
use crossterm::cursor;
use crossterm::style::ResetColor;
use crossterm::ExecutableCommand;

use crate::words;
use crate::ui;

pub fn run_game() -> Result<(), Box<dyn std::error::Error>> {
    let target_words = crate::words::get_random_words(10, 12345, "basic");
    let target_text = target_words.join(" ");

    let mut stdout = std::io::stdout();
    ui::init_terminal()?;
    ui::draw_prompt(&target_text)?;
    stdout.flush()?;

    let mut typed = String::new();
    let start = Instant::now();

    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) => typed.push(c),
                    KeyCode::Backspace => { typed.pop(); }
                    KeyCode::Esc | KeyCode::Enter => break,
                    _ => {}
                }

                ui::draw_word_stream(&target_text, &typed)?;
            }
        }

        if typed.len() >= target_text.len() {
            break;
        }
    }

    let duration = start.elapsed();
    let wpm = calculate_wpm(&typed, duration);
    let accuracy = calculate_accuracy(&target_text, &typed);

    stdout.execute(cursor::MoveTo(0, 5))?;
    stdout.execute(ResetColor)?;
    ui::cleanup_terminal()?;
    println!("\n--- Results ---");
    println!("Time: {:.2?}", duration);
    println!("WPM: {:.2}", wpm);
    println!("Accuracy: {:.2}%", accuracy * 100.0);

    Ok(())
}

fn calculate_wpm(input: &str, duration: Duration) -> f64 {
    let words = input.split_whitespace().count() as f64;
    let minutes = duration.as_secs_f64() / 60.0;
    if minutes > 0.0 { words / minutes } else { 0.0 }
}

fn calculate_accuracy(expected: &str, actual: &str) -> f64 {
    let total_chars = expected.len().max(actual.len());
    let correct_chars = expected
        .chars()
        .zip(actual.chars())
        .filter(|(a, b)| a == b)
        .count();

    if total_chars > 0 {
        correct_chars as f64 / total_chars as f64
    } else {
        1.0
    }
}