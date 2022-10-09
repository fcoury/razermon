#[cfg(any(target_os = "macos"))]
use crate::battery::BatteryStatus;
use razermacos::devices::USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS;
use std::{thread, time::Duration};
use tauri::{CustomMenuItem, Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu};

mod battery;

fn main() {
    let status =
        if let Some(status) = BatteryStatus::get(USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS) {
            status.to_string()
        } else {
            "".to_string()
        };
    let items = SystemTrayMenu::new().add_item(CustomMenuItem::new("quit", "Quit"));

    #[allow(unused_mut)]
    let mut app = tauri::Builder::default()
        .system_tray(SystemTray::new().with_title(&status).with_menu(items))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let _item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "battery" => {
                        let status =
                            BatteryStatus::get(USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS);
                        if let Some(status) = status {
                            app.tray_handle()
                                .get_item("battery")
                                .set_title(status.to_string())
                                .unwrap();
                        }
                    }
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

    let handle = app.handle().clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        let status = BatteryStatus::get(USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS);
        if let Some(status) = status {
            handle.tray_handle().set_title(&status.to_string()).unwrap();
        }
    });

    app.run(move |_app_handle, e| {
        if let RunEvent::ExitRequested { api, .. } = &e {
            api.prevent_exit();
        }
        // if let Some(on_event) = &mut on_event {
        //     (on_event)(app_handle, e);
        // }
    });
}
