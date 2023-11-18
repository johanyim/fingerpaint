pub mod color;
pub mod commands;
pub mod config;
pub mod error;

pub mod palette;
pub use palette::reader;

pub mod ui;
pub use ui::keyboard;
pub use ui::window;
pub use ui::tabline;


