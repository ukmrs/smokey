//! ```text
//!   _._ _  _ |  _    
//!  _>| | |(_)|<(/_\/
//!                 /  
//! ```
//! by ukmrs https://github.com/ukmrs/smokey
//! A simple typing test terminal UI app

use smokey::{application::App, database, storage};

use std::io::stdout;
use structopt::StructOpt;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> crossterm::Result<()> {
    let opt = Opt::from_args();
    if execute_info_requests(&opt) {
        return Ok(());
    }

    #[allow(unused_mut)]
    let mut sout = stdout();

    let backend = CrosstermBackend::new(sout);
    let terminal = Terminal::new(backend)?;

    let app = App::from_config();

    smokey::run(app, terminal)?;
    Ok(())
}

#[derive(StructOpt)]
struct Opt {
    /// Prints expected path of the smokey config file
    #[structopt(short, long)]
    config: bool,

    /// Prints path of the smokey storage directory
    #[structopt(short, long)]
    storage: bool,

    /// Prints out summaries of n most recent runs
    #[structopt(short, long, name = "n")]
    recent: Option<Option<usize>>,
}

fn execute_info_requests(opt: &Opt) -> bool {
    let mut should_exit: bool = false;
    if opt.storage {
        should_exit = true;
        println!("{}", storage::get_storage_dir().to_str().unwrap())
    }

    if opt.config {
        should_exit = true;
        println!("{}", storage::get_config_file().to_str().unwrap())
    }

    if let Some(us) = opt.recent {
        let history_lines = us.unwrap_or(12);
        should_exit = true;
        database::RunHistoryDatbase::default().print_history(history_lines);
    }

    should_exit
}
