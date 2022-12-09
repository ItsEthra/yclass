use std::ops::Deref;

use eframe::{
    egui::{Context, InputState, Key, KeyboardShortcut, Modifiers},
    epaint::ahash::HashMap,
};

#[derive(Default)]
pub struct HotkeyManager {
    names: HashMap<&'static str, KeyboardShortcut>,
}

impl HotkeyManager {
    pub fn register(&mut self, name: &'static str, key: Key, modifiers: Modifiers) -> &mut Self {
        self.names.insert(name, KeyboardShortcut { key, modifiers });

        self
    }

    pub fn pressed(&self, name: &'static str, input: impl Deref<Target = InputState>) -> bool {
        let Some(shortcut) = self.names.get(name) else {
            return false;
        };

        input.key_pressed(shortcut.key) && input.modifiers.matches(shortcut.modifiers)
    }

    pub fn format(&self, name: &'static str, ctx: &Context) -> String {
        let shortcut = self
            .names
            .get(name)
            .expect("No shortcut registered for this name");
        ctx.format_shortcut(shortcut)
    }
}
