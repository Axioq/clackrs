use std::io::Write;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyCode};
use crossterm::cursor;
use crossterm::style::ResetColor;
use crossterm::ExecutableCommand;

use crate::ui;

pub fn run_game(duration: Duration) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    ui::init_terminal()?;

    let mut typed = String::new();
    let mut all_words: Vec<String> = crate::words::get_random_words(100, rand::random(), "basic");
    let mut word_window_start = 0;
    let words_per_window = 15;

    let start = Instant::now();
    let end_time = start + duration;

    loop {
        if Instant::now() >= end_time {
            break;
        }

        // Build the prompt line (current window of words)
        let word_window_end = (word_window_start + words_per_window).min(all_words.len());
        let visible_words = &all_words[word_window_start..word_window_end];
        let prompt_text = visible_words.join(" ");

        ui::draw_prompt(&prompt_text)?;
        stdout.flush()?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) => typed.push(c),
                    KeyCode::Backspace => {
                        typed.pop();
                    }
                    KeyCode::Esc => return Ok(()),
                    _ => {}
                }

                let time_left = end_time.saturating_duration_since(Instant::now()).as_secs();
                ui::draw_word_stream(&prompt_text, &typed, time_left)?;

                // Check if user is near end of visible words
                let typed_word_count = typed.split_whitespace().count();
                if typed_word_count > word_window_start + (words_per_window as f64 * 0.75) as usize {
                    word_window_start += 5;
                    if word_window_end + 5 > all_words.len() {
                        all_words.extend(crate::words::get_random_words(20, rand::random(), "basic"));
                    }
                }
            }
        }
    }

    let duration = start.elapsed();
    let full_prompt = all_words.join(" ");
    let accuracy = calculate_accuracy(&full_prompt[..typed.len().min(full_prompt.len())], &typed);
    let wpm = calculate_wpm(&typed, duration);

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