//! # painters
//! contains **Painter** functions of the following signature
//! ```
//! use smokey::{application::App, Term};
//! pub type Painter = fn(terminal: &mut Term, app: &mut App);
//! ```
//! the idea is to assign the pointer to specific Painter function
//! once in a while when it's appropriate instead of checking
//! against app state all the time to determine which one to use
//!
//! All painters had been trained using Bob Ross' *The Joy of Painting*

mod post;
mod settings;
mod test;

use crate::{application::App, Term};

// re-exports
pub use post::draw_post;
pub use settings::draw_settings;
pub use test::draw_test;

/// Signature of a function responsible for drawing to the terminal
pub type Painter = fn(terminal: &mut Term, app: &mut App);
