// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{
    menu::{Menu, MenuItem},
    Manager, Window, Wry,
};

struct State {
    menu1: Option<Menu<Wry>>,
    menu2: Option<Menu<Wry>>,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![menu1, menu2])
        .setup(|app| {
            app.manage(Mutex::new(State {
                menu1: None,
                menu2: None,
            }));

            let manager = app.handle();

            let menu1 = Menu::with_items(
                manager,
                &[&MenuItem::with_id(
                    manager,
                    "menu1item1",
                    "menu1item1",
                    true,
                    None::<&str>,
                )?],
            )?;

            let menu2 = Menu::with_items(
                manager,
                &[&MenuItem::with_id(
                    manager,
                    "menu2item1",
                    "menu2item1",
                    true,
                    None::<&str>,
                )?],
            )?;

            let state = manager.state::<Mutex<State>>();
            *state.lock().unwrap() = State {
                menu1: Some(menu1),
                menu2: Some(menu2),
            };

            let window = app.get_webview_window("main").unwrap();
            window.on_menu_event(|w, e: tauri::menu::MenuEvent| {
                w.emit("menuitem", e.id().0.as_str()).unwrap();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn menu1(window: Window) {
    let state = window.state::<Mutex<State>>();
    let menus = state.lock().unwrap();
    window.popup_menu(menus.menu1.as_ref().unwrap()).unwrap();
}

#[tauri::command]
fn menu2(window: Window) {
    let state = window.state::<Mutex<State>>();
    let menus = state.lock().unwrap();
    window.popup_menu(menus.menu2.as_ref().unwrap()).unwrap();
}
