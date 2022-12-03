#[cfg(all(not(unix), not(windows)))]
compile_error!("Only UNIX and Windows platforms are supported");

mod app;
mod class;
mod gui;
mod state;

use eframe::{NativeOptions, Theme};
use state::GlobalState;
use std::cell::RefCell;
use yclass_config::YClassConfig;

fn main() {
    eframe::run_native(
        "YClass",
        NativeOptions {
            default_theme: Theme::Dark,
            ..Default::default()
        },
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.5);

            Box::new(app::YClassApp::new(Box::leak(Box::new(RefCell::new(
                GlobalState {
                    config: YClassConfig::load_or_default(),
                    ..Default::default()
                },
            )))))
        }),
    )
}
