#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

#[cfg(all(not(unix), not(windows)))]
compile_error!("Only UNIX and Windows platforms are supported.");

#[cfg(not(target_pointer_width = "64"))]
compile_error!("Only X64 targets are supported.");

mod address;
mod app;
mod class;
mod config;
mod context;
mod field;
mod generator;
mod gui;
mod process;
mod project;
mod state;

use eframe::{
    egui::{FontData, FontDefinitions},
    epaint::{FontFamily, FontId},
    NativeOptions, Theme,
};
use std::time::Duration;

/// Monospaced font id.
const FID_M: FontId = FontId::monospace(20.);

fn main() {
    eframe::run_native(
        "YClass",
        NativeOptions {
            default_theme: Theme::Dark,
            ..Default::default()
        },
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.25);

            let mut fonts = FontDefinitions::default();
            fonts.font_data.insert(
                "roboto-mono".into(),
                FontData::from_static(include_bytes!("../../fonts/RobotoMono-Regular.ttf")),
            );
            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .insert(0, "roboto-mono".into());
            cc.egui_ctx.set_fonts(fonts);
            cc.egui_ctx
                .request_repaint_after(Duration::from_millis(100));

            Box::new(app::YClassApp::new(Box::leak(Box::default())))
        }),
    )
}
