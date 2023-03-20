#[cfg(any(target_os = "macos"))]
use crate::battery::BatteryStatus;
use battery::BatteryData;
use razer_driver_rs::scan_for_devices;
use std::{thread, time::Duration};
use tauri::{
    api::notification::Notification, AppHandle, CustomMenuItem, Manager, RunEvent, SystemTray,
    SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

mod battery;
mod database;
mod human_display;
mod settings;

fn main() {
    let product_id = load_product_id();
    let menu = tray_menu(product_id);
    let status = status(product_id);

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
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "battery" => {
                        let product_id = settings::get("product_id").unwrap().unwrap();
                        let status = BatteryStatus::get(product_id.parse().unwrap());
                        if let Ok(Some(status)) = status {
                            app.tray_handle()
                                .get_item("battery")
                                .set_title(status.to_string())
                                .unwrap();
                        } else {
                            app.tray_handle()
                                .get_item("battery")
                                .set_title("No battery data".to_string())
                                .unwrap();
                        }
                    }
                    "usage" => {
                        let window = app.get_window("main").unwrap();
                        window.show().unwrap();
                        window.eval("window.location.reload()").unwrap();
                        window.set_focus().unwrap();
                    }
                    "no_devices" => {
                        eprintln!("No devices clicked");
                    }
                    "devtools" => {
                        #[cfg(debug_assertions)]
                        app.get_window("main").unwrap().open_devtools();
                    }
                    "notify" => {
                        Notification::new(&app.config().tauri.bundle.identifier)
                            .icon("icons/128x128.png")
                            .title("Notification test")
                            .body("If you are seeing this text, it worked 😊")
                            .show()
                            .unwrap();
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    str => {
                        if str.starts_with("device_") {
                            let res = scan_for_devices(None).unwrap();
                            let devices = res.devices;
                            if !devices.is_empty() {
                                let str_id = str.replace("device_", "");
                                settings::set("product_id", &str_id).unwrap();
                                let id: u16 = str_id.parse().unwrap();
                                app.tray_handle();
                                item_handle.set_selected(true).unwrap();
                                for device_spec in devices {
                                    if device_spec.device.product_id() != id {
                                        app.tray_handle()
                                            .get_item(&format!(
                                                "device_{}",
                                                device_spec.device.product_id()
                                            ))
                                            .set_selected(false)
                                            .unwrap();
                                    }
                                }
                                update_tray_display(app, id);
                            }
                        }
                    }
                }
            }
            _ => {}
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            charge_history,
            selected_product_id,
            device_status,
            battery_stats,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    // #[cfg(target_os = "macos")]
    // app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    let handle = app.handle();
    if let Some(product_id) = product_id {
        start_updates(handle, product_id);
    }

    app.run(move |_app_handle, e| {
        if let RunEvent::ExitRequested { api, .. } = &e {
            api.prevent_exit();
        }
    });
}

#[tauri::command]
fn selected_product_id() -> Option<u16> {
    load_product_id()
}

#[tauri::command]
fn device_status(product_id: u16) -> Option<BatteryStatus> {
    BatteryStatus::get(product_id).unwrap()
}

#[tauri::command]
fn charge_history(product_id: u16) -> Result<Vec<BatteryData>, String> {
    match BatteryData::get(product_id) {
        Ok(data) => Ok(data),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn battery_stats(product_id: u16) -> Result<Option<(i64, Option<String>)>, String> {
    match BatteryData::get(product_id) {
        Ok(data) => match BatteryData::consumption(&data) {
            Some(consumption) => Ok(Some((consumption, remaining(Some(product_id))))),
            None => Ok(None),
        },
        Err(err) => Err(err.to_string()),
    }
}

fn status(product_id: Option<u16>) -> String {
    if let Some(product_id) = product_id {
        if let Ok(Some(status)) = BatteryStatus::get(product_id) {
            status.to_string()
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    }
}

fn load_product_id() -> Option<u16> {
    let product_id = settings::get("product_id").unwrap();
    let product_id = match product_id {
        Some(product_id) => Some(product_id.parse().unwrap()),
        None => {
            let res = scan_for_devices(None).unwrap();
            if res.devices.is_empty() {
                return None;
            }
            let Some(device) = res.devices.get(0) else {
                return None;
            };
            let product_id = device.device.product_id();
            settings::set("product_id", &product_id.to_string()).unwrap();
            Some(product_id)
        }
    };
    product_id
}

fn start_updates(handle: AppHandle, mut product_id: u16) {
    let mut curr_percentage = BatteryStatus::last_status(product_id).unwrap().unwrap_or(0);
    let mut notified = false;
    let mut curr_product_id = product_id;
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));

        let res = match scan_for_devices(None) {
            Ok(result) => result,
            Err(err) => {
                eprintln!("Error scanning for devices: {}", err);
                continue;
            }
        };
        let devices = res.devices;
        let device_connected = devices.iter().any(|d| d.device.product_id() == product_id);

        if !device_connected {
            let new_device = devices.get(0);
            println!("Device connected: {:?}", new_device);
            if let Some(new_device) = new_device {
                product_id = new_device.device.product_id();
                settings::set("product_id", &product_id.to_string()).unwrap();
                update_tray_display(&handle, product_id);
            } else {
                // No devices found, update the tray display accordingly
                handle.tray_handle().set_title("No devices found").unwrap();
                handle
                    .tray_handle()
                    .get_item("remaining")
                    .set_title("No devices found")
                    .unwrap();
                continue;
            }
        }

        match BatteryStatus::get_from_devices(&devices, product_id) {
            Ok(Some(status)) => {
                if status.percentage != curr_percentage || curr_product_id != product_id {
                    curr_percentage = status.percentage;
                    curr_product_id = product_id;
                    handle.tray_handle().set_title(&status.to_string()).unwrap();
                    let res = status.save();
                    if res.is_err() {
                        eprintln!("WARN: Couldn't save battery status");
                    }

                    // notifies again at 5%
                    if status.percentage < 5 {
                        notified = false;
                    }

                    // if battery is below threshold and not already notified
                    if status.percentage < 10 && status.percentage != 0 && !notified {
                        Notification::new("org.fcoury.razermon")
                            .icon("icons/128x128.png")
                            .title("Battery warning")
                            .body("Your battery is running low.")
                            .show()
                            .unwrap();
                        notified = true;
                    }
                }
            }
            Ok(None) => {
                handle.tray_handle().set_title("No devices found").unwrap();
                handle
                    .tray_handle()
                    .get_item("remaining")
                    .set_title("No devices found")
                    .unwrap();
            }
            Err(err) => {
                eprintln!("Error getting battery status: {}", err);
            }
        }
    });
}

fn remaining(product_id: Option<u16>) -> Option<String> {
    if let Some(product_id) = product_id {
        match BatteryStatus::get(product_id) {
            Ok(Some(status)) => {
                if let Ok(remaining) = status.fmt_remaining() {
                    return remaining.map(|r| format!("{} remaining", r));
                }
            }
            Ok(None) => {}
            Err(err) => {
                eprintln!("Error getting battery status: {}", err);
            }
        }
    }
    None
}

fn tray_menu(product_id: Option<u16>) -> SystemTrayMenu {
    let mut menu = SystemTrayMenu::new();

    let remaining = match remaining(product_id) {
        Some(remaining) => remaining,
        None => "Not enough data to calulate ETA yet".to_string(),
    };

    let mut remaining_item = CustomMenuItem::new("remaining", remaining);
    remaining_item.enabled = false;
    menu = menu
        .add_item(remaining_item)
        .add_native_item(SystemTrayMenuItem::Separator);

    if let Some(product_id) = product_id {
        let res = scan_for_devices(None).unwrap();
        let devices = res.devices;
        if devices.is_empty() {
            return no_devices_menu(&menu);
        }
        menu = menu
            .add_item(CustomMenuItem::new("usage", "Usage Chart..."))
            .add_item(CustomMenuItem::new("notify", "Test Notification"))
            .add_native_item(SystemTrayMenuItem::Separator);

        for device_spec in devices {
            let id = format!("device_{}", device_spec.device.product_id());
            let mut item = CustomMenuItem::new(id, &device_spec.name);
            item.selected = product_id == device_spec.device.product_id();
            // item.enabled = device.has_battery(); TODO
            menu = menu.add_item(item);
        }

        menu = menu
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new("devtools", "Open DevTools"));
    } else {
        menu = no_devices_menu(&menu);
    };

    menu.add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit"))
}

fn no_devices_menu(menu: &SystemTrayMenu) -> SystemTrayMenu {
    let menu = menu.clone();
    let mut item = CustomMenuItem::new("no_devices", "No devices found");
    item.enabled = false;
    menu.add_item(item)
}

fn update_tray_display(handle: &AppHandle, product_id: u16) {
    match BatteryStatus::get(product_id) {
        Ok(Some(status)) => {
            handle.tray_handle().set_title(&status.to_string()).unwrap();
        }
        Ok(None) => {
            handle.tray_handle().set_title("No battery data").unwrap();
        }
        Err(err) => {
            eprintln!("Error getting battery status: {}", err);
            handle
                .tray_handle()
                .set_title("Error getting battery status")
                .unwrap();
        }
    }

    if let Some(remaining) = remaining(Some(product_id)) {
        handle
            .tray_handle()
            .get_item("remaining")
            .set_title(remaining)
            .unwrap();
    }
}
