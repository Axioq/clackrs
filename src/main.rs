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

    // Show main menu and handle selection
    let mode = match ui::show_menu() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error displaying menu: {}", e);
            let _ = ui::cleanup_terminal();
            return;
        }
    };

    match mode {
        ui::GameMode::SinglePlayer => {
            if let Err(e) = ui::wait_for_enter() {
                eprintln!("Error: {}", e);
                let _ = ui::cleanup_terminal();
                return;
            }
            match game::run_game(std::time::Duration::from_secs(60)) {
                Ok(_) => println!("Thanks for playing!"),
                Err(e) => eprintln!("Game error: {}", e),
            }
        }
        ui::GameMode::Multiplayer => {
            println!("Multiplayer mode coming soon!");
        }
        ui::GameMode::Exit => {
            // Just exit
        }
    }

    // Clean up terminal
    if let Err(e) = ui::cleanup_terminal() {
        eprintln!("Error cleaning up terminal: {}", e);
    }
}