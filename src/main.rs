mod game;
mod ui;
mod stats;
mod words;

fn main() {
    // Initialize terminal
    if let Err(e) = ui::init_terminal() {
        eprintln!("Error initializing terminal: {}", e);
        return;
    }

    // Run the game loop
    match game::run_game() {
        Ok(_) => println!("Thanks for playing!"),
        Err(e) => eprintln!("Game error: {}", e),
    }

    // Clean up terminal
    if let Err(e) = ui::cleanup_terminal() {
        eprintln!("Error cleaning up terminal: {}", e);
    }
}