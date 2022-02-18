use std::time::Duration;

use tui::{backend::TermionBackend, Terminal};
use termion::{raw::IntoRawMode, input::TermRead};
use structopt::StructOpt;


mod fclock;
mod fblock;
use fclock::FClock;


#[derive(StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct OptionalArgs {
    #[structopt(short = "c", long = "centered")]
    centered: bool,

    #[structopt(short = "s", long = "size")]
    size: Option<Vec<u16>>,

    #[structopt(short = "p", long = "position")]
    pos: Option<Vec<u16>>,

    #[structopt(short = "b", long = "borders")]
    borders: bool,
}


fn main() -> Result<(), std::io::Error> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let mut stdin_keys = termion::async_stdin().keys();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    
    // Before main loop
    terminal.clear()?;
    //terminal.autoresize()?;
    let mut clock = FClock::from_optional_args();
    
    // Main loop
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
        
        std::thread::sleep(Duration::from_secs(1));
    }
    

    // After main loop
    terminal.clear()?;
    //terminal.autoresize()?;


    Ok(())
}