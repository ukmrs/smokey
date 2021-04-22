mod post;
mod settings;
mod test;

pub use post::draw_post;
pub use settings::draw_settings;
pub use test::draw_test;

use tui::{backend::Backend, Terminal};
use crate::application::App;

pub type Painter<B: Backend> = fn(terminal: &mut Terminal<B>, app: &mut App);
