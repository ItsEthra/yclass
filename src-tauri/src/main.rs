#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{async_runtime::Mutex, CustomMenuItem, Menu, State, Submenu};
use yclass_core::{ProcessEntry, ProcessInterface, Result};

#[derive(Default)]
struct Globals {
    process: Mutex<Option<Box<dyn ProcessInterface>>>,
}

#[tauri::command]
fn list_processes() -> Result<Vec<ProcessEntry>> {
    yclass_core::fetch_processes()
}

#[tauri::command]
async fn attach(pid: u32, state: State<Globals, '_>) -> Result<()> {
    let process = yclass_core::attach(pid)?;
    *state.process.lock().await = Some(process);

    Ok(())
}

#[tauri::command]
async fn detach(state: State<Globals, '_>) -> Result<()> {
    *state.process.lock().await = None;

    Ok(())
}

fn create_menu() -> Menu {
    let mut process_attach_recent =
        CustomMenuItem::new("process_attach_recent", "Attach to recent");
    process_attach_recent.enabled = false;

    let process = Menu::new()
        .add_item(CustomMenuItem::new("process_attach", "Attach to process"))
        .add_item(process_attach_recent)
        .add_item(CustomMenuItem::new("process_detach", "Detach from process"));

    Menu::new().add_submenu(Submenu::new("Process", process))
}

fn main() {
    let menu = create_menu();

    tauri::Builder::default()
        .menu(menu)
        .manage(Globals::default())
        .on_menu_event(|event| match event.menu_item_id() {
            "process_attach" => {}
            _ => unreachable!(),
        })
        .invoke_handler(tauri::generate_handler![list_processes, attach, detach])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
