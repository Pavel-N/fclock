// ===== Imports ====
use tui::{backend::TermionBackend, Terminal};
use termion::{input::TermRead, raw::IntoRawMode};

// ===== Modules ====
mod fclock;
mod fblock;
mod opt_args;

// ===== Crate imports ====
use fclock::FClock;


fn main() -> Result<(), std::io::Error> {
    
    // ===== Before main loop =====
    // Create clock
    let mut clock = FClock::from_optional_args();
    // Create new terminal with raw mode
    let mut terminal = Terminal::new(
        TermionBackend::new(std::io::stdout().into_raw_mode()?)
    )?;
    // Crete asynchronous standard input that listens to keys
    let mut stdin_keys = termion::async_stdin().keys();
    // Clear terminal before drawing clock
    terminal.clear()?;


    // ===== Main loop =====
    'main: loop {
        // Update
        clock.update();
        
        // Draw
        terminal.draw(|f| clock.draw(f))?;
        
        // Handle events
        for key in stdin_keys.next() {
            use termion::event::Key;
        
            match key.unwrap() {
                Key::Char('q') | Key::Char('Q') | Key::Esc => break 'main,
                _ => {}
            }
        }
        
        // Wait before next "frame"
        std::thread::sleep(std::time::Duration::from_millis(200));  // HACK: Not ideal
    }
    

    // ===== After main loop =====
    terminal.clear()?;


    Ok(())
}