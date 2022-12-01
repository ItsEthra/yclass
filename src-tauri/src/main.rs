#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod process_selector;

use tauri::{CustomMenuItem, Manager, Menu, Submenu, WindowBuilder, WindowUrl};

fn project_submenu() -> Submenu {
    Submenu::new(
        "Project",
        Menu::new()
            .add_item(CustomMenuItem::new("pj_new", "Create new project"))
            .add_item(CustomMenuItem::new("pj_open", "Open project"))
            .add_item(CustomMenuItem::new("pj_save", "Save project"))
            .add_item(CustomMenuItem::new("pj_save_as", "Save project as")),
    )
}

fn process_submenu() -> Submenu {
    Submenu::new(
        "Process",
        Menu::new()
            .add_item(CustomMenuItem::new("ps_attach", "Attach to process"))
            .add_item(CustomMenuItem::new("ps_reattach", "Attach to last process"))
            .add_item(CustomMenuItem::new("ps_detach", "Detach from process")),
    )
}

fn main() {
    let main_menu = Menu::new()
        .add_submenu(project_submenu())
        .add_submenu(process_submenu());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            process_selector::fetch_all_processes
        ])
        .setup(|app| {
            WindowBuilder::new(&app.handle(), "main", WindowUrl::App("index.html".into()))
                .title("YClass")
                .menu(main_menu)
                .build()?;

            Ok(())
        })
        .on_menu_event(move |event| {
            match event.menu_item_id() {
                "ps_attach" => {
                    _ = WindowBuilder::new(
                        &event.window().app_handle(),
                        "wnd_process_select",
                        WindowUrl::App("index.html".into()),
                    )
                    .title("YClass - Attach process")
                    .inner_size(320., 480.)
                    .build()
                    .unwrap();
                }
                _ => {}
            };
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
