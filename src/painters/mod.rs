//! # painters
//! contains **Painter** functions of the following signature
//! ```
//! use tui::Terminal;
//! use smokey::application::App;
//! pub type Painter<B> = fn(terminal: &mut Terminal<B>, app: &mut App);
//! ```
//! the idea is to assign the pointer to specific Painter function
//! once in a while when it's appropriate instead of checking
//! against app state all the time to determine which one to use
//!
//! All painters had been trained using Bob Ross' *The Joy of Painting*

mod post;
mod settings;
mod test;

pub use post::draw_post;
pub use settings::draw_settings;
pub use test::draw_test;

use crate::application::App;
use tui::Terminal;

pub type Painter<B> = fn(terminal: &mut Terminal<B>, app: &mut App);
