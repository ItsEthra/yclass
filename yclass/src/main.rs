#[cfg(all(not(unix), not(windows)))]
compile_error!("Only UNIX and Windows platforms are supported");

use eframe::{NativeOptions, Theme};

mod app;
mod gui;

fn main() {
    eframe::run_native(
        "YClass",
        NativeOptions {
            default_theme: Theme::Dark,
            ..Default::default()
        },
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.5);

            Box::<app::YClassApp>::default()
        }),
    )
}
