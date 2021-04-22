mod post;
mod settings;
mod test;

pub use post::draw_post;
pub use settings::draw_settings;
pub use test::draw_test;

use crate::application::App;
use tui::Terminal;

pub type Painter<B> = fn(terminal: &mut Terminal<B>, app: &mut App);
