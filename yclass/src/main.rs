#[cfg(all(not(unix), not(windows)))]
compile_error!("Only UNIX and Windows platforms are supported");

mod address;
mod app;
mod class;
mod gui;
mod state;

use eframe::{
    egui::{FontData, FontDefinitions},
    epaint::FontFamily,
    NativeOptions, Theme,
};

fn main() {
    eframe::run_native(
        "YClass",
        NativeOptions {
            default_theme: Theme::Dark,
            ..Default::default()
        },
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.5);

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

            Box::new(app::YClassApp::new(Box::leak(Box::default())))
        }),
    )
}
