mod class_list;
pub use class_list::*;

mod tool_bar;
pub use tool_bar::*;

mod process_attach;
pub use process_attach::*;

mod inspector;
pub use inspector::*;

use eframe::epaint::FontId;

const FID_M: FontId = FontId::monospace(18.);
