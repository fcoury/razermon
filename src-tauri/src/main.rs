#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use razermacos::{devices::USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS, RazerDevices};
use tauri::{CustomMenuItem, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu};

fn main() {
    let mut devices = RazerDevices::all();
    let device = devices.find(USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS as u16);

    let item = if let Some(device) = device {
        let ch_str = if device.is_charging() { " ⚡️" } else { "" };
        CustomMenuItem::new(
            "battery",
            format!("Battery: 🔋{}%{}", device.battery(), ch_str),
        )
    } else {
        CustomMenuItem::new("battery", "No devices found.")
    };
    let items = SystemTrayMenu::new().add_item(item);

    #[allow(unused_mut)]
    let mut app = tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(items))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                // let window = app.get_window("main").unwrap();
                // window.show().unwrap();
                // window.set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let _item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "no_devices" => {
                        eprintln!("No devices clicked");
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(move |_app_handle, e| {
        if let RunEvent::ExitRequested { api, .. } = &e {
            // Keep the event loop running even if all windows are closed
            // This allow us to catch system tray events when there is no window
            api.prevent_exit();
        }
        // if let Some(on_event) = &mut on_event {
        //     (on_event)(app_handle, e);
        // }
    });
}
