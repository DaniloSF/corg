#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
pub mod parser;
mod statics;
pub mod types;

use types::RoaList;

use std::sync::Mutex;

use tauri::api::shell;

use tauri::{
    CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu, WindowBuilder, WindowUrl,
};

fn main() {
    let ctx = tauri::generate_context!();

    tauri::Builder::default()
        .manage(types::InnerState {
            store: Mutex::new(RoaList::new()),
        })
        .invoke_handler(tauri::generate_handler![
            commands::read_roa_file,
            commands::greet,
            commands::get_characters_tree,
            commands::get_roa_list,
            commands::move_categories
        ])
        .setup(|app| {
            let _window = WindowBuilder::new(app, "main", WindowUrl::default())
                .title("Corg")
                .inner_size(800.0, 550.0)
                .min_inner_size(400.0, 200.0)
                .disable_file_drop_handler()
                .build()
                .expect("Unable to create window");
            Ok(())
        })
        .menu(Menu::with_items([
            #[cfg(target_os = "macos")]
            MenuEntry::Submenu(Submenu::new(
                &ctx.package_info().name,
                Menu::with_items([
                    MenuItem::About(ctx.package_info().name.clone(), AboutMetadata::default())
                        .into(),
                    MenuItem::Separator.into(),
                    MenuItem::Services.into(),
                    MenuItem::Separator.into(),
                    MenuItem::Hide.into(),
                    MenuItem::HideOthers.into(),
                    MenuItem::ShowAll.into(),
                    MenuItem::Separator.into(),
                    MenuItem::Quit.into(),
                ]),
            )),
            MenuEntry::Submenu(Submenu::new(
                "File",
                Menu::with_items([MenuItem::CloseWindow.into()]),
            )),
            MenuEntry::Submenu(Submenu::new(
                "Edit",
                Menu::with_items([
                    MenuItem::Undo.into(),
                    MenuItem::Redo.into(),
                    MenuItem::Separator.into(),
                    MenuItem::Cut.into(),
                    MenuItem::Copy.into(),
                    MenuItem::Paste.into(),
                    #[cfg(not(target_os = "macos"))]
                    MenuItem::Separator.into(),
                    MenuItem::SelectAll.into(),
                ]),
            )),
            MenuEntry::Submenu(Submenu::new(
                "View",
                Menu::with_items([MenuItem::EnterFullScreen.into()]),
            )),
            MenuEntry::Submenu(Submenu::new(
                "Window",
                Menu::with_items([MenuItem::Minimize.into(), MenuItem::Zoom.into()]),
            )),
            // You should always have a Help menu on macOS because it will automatically
            // show a menu search field
            MenuEntry::Submenu(Submenu::new(
                "Help",
                Menu::with_items([CustomMenuItem::new("Learn More", "Learn More").into()]),
            )),
        ]))
        .on_menu_event(|event| {
            let event_name = event.menu_item_id();
            match event_name {
                "Learn More" => {
                    let url = "https://github.com/probablykasper/tauri-template".to_string();
                    shell::open(&event.window().shell_scope(), url, None).unwrap();
                }
                _ => {}
            }
        })
        .run(ctx)
        .expect("error while running tauri application");
}
