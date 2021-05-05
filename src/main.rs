//! ```text
//!   _._ _  _ |  _    
//!  _>| | |(_)|<(/_\/
//!                 /  
//! ```
//! by ukmrs https://github.com/ukmrs/smokey
//! A simple typing test terminal UI app

use smokey::application::App;
use smokey::colorscheme::Theme;
use std::io::stdout;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> crossterm::Result<()> {
    #[allow(unused_mut)]
    let mut sout = stdout();
    let backend = CrosstermBackend::new(sout);
    // TODO Theme is all over the place
    // best to integrate into the app struct
    
    let theme = Theme::new();
    let terminal = Terminal::new(backend)?;
    let app = App::new();

    smokey::run(app, theme, terminal)?;
    Ok(())
}
