#[cfg(any(target_os = "macos"))]
use crate::battery::BatteryStatus;
use razermacos::devices::USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS;
use std::{thread, time::Duration};
use tauri::{
    CustomMenuItem, Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

mod battery;
mod database;

fn devices() -> Option<Vec<String>> {
    let mut devices = razermacos::RazerDevices::new();
    match devices.all() {
        Some(devices) => Some(devices.iter().map(|device| device.name.clone()).collect()),
        None => None,
    }
}

fn main() {
    let status =
        if let Some(status) = BatteryStatus::get(USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS) {
            status.to_string()
        } else {
            "".to_string()
        };
    let mut menu = SystemTrayMenu::new();
    if let Some(names) = devices() {
        for name in names {
            let item = CustomMenuItem::new("device", name);
            // item.selected = true;
            menu = menu.add_item(item);
        }
    }
    menu = menu
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit"));

    #[allow(unused_mut)]
    let mut app = tauri::Builder::default()
        .system_tray(SystemTray::new().with_title(&status).with_menu(menu))
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

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    let handle = app.handle().clone();
    let mut curr_percentage = BatteryStatus::last_status().unwrap().unwrap_or(0);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        let status = BatteryStatus::get(USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS);
        if let Some(status) = status {
            handle.tray_handle().set_title(&status.to_string()).unwrap();
            if status.percentage != curr_percentage {
                let res = status.save();
                if res.is_err() {
                    eprintln!("WARN: Couldn't save battery status");
                }
                curr_percentage = status.percentage;
            }
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
