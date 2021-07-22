//! ```text
//!   _._ _  _ |  _    
//!  _>| | |(_)|<(/_\/
//!                 /  
//! ```
//! by ukmrs https://github.com/ukmrs/smokey
//! A simple typing test terminal UI app

use smokey::application::App;
use std::io::stdout;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> crossterm::Result<()> {
    #[allow(unused_mut)]
    let mut sout = stdout();

    let backend = CrosstermBackend::new(sout);
    let terminal = Terminal::new(backend)?;

    let app = App::from_config();

    smokey::run(app, terminal)?;
    Ok(())
}
