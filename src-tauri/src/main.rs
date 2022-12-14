#[cfg(any(target_os = "macos"))]
use crate::battery::BatteryStatus;
use battery::BatteryData;
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
                        if let Some(status) = status {
                            app.tray_handle()
                                .get_item("battery")
                                .set_title(status.to_string())
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
                            .body("If you are seeing this text, it worked ????")
                            .show()
                            .unwrap();
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    str => {
                        if str.starts_with("device_") {
                            if let Some(devices) = razermacos::RazerDevices::new().all() {
                                let str_id = str.replace("device_", "");
                                settings::set("device", &str_id).unwrap();
                                let id: u16 = str_id.parse().unwrap();
                                app.tray_handle();
                                item_handle.set_selected(true).unwrap();
                                for device in devices {
                                    if device.product_id() != id {
                                        app.tray_handle()
                                            .get_item(&format!("device_{}", device.product_id()))
                                            .set_selected(false)
                                            .unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
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

    let handle = app.handle().clone();
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
    BatteryStatus::get(product_id)
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
    let status = if let Some(product_id) = product_id {
        if let Some(status) = BatteryStatus::get(product_id) {
            status.to_string()
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    };
    status
}

fn load_product_id() -> Option<u16> {
    let product_id = settings::get("product_id").unwrap();
    let product_id = match product_id {
        Some(product_id) => Some(product_id.parse().unwrap()),
        None => {
            if let Some(devices) = razermacos::RazerDevices::new().all() {
                let product_id = devices
                    .iter()
                    .find(|d| d.has_battery())
                    .unwrap()
                    .product_id();
                settings::set("product_id", &product_id.to_string()).unwrap();
                Some(product_id)
            } else {
                None
            }
        }
    };
    product_id
}

fn start_updates(handle: AppHandle, product_id: u16) {
    let mut curr_percentage = BatteryStatus::last_status(product_id).unwrap().unwrap_or(0);
    let mut notified = false;
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        let status = BatteryStatus::get(product_id);
        if let Some(status) = status {
            handle.tray_handle().set_title(&status.to_string()).unwrap();
            if status.percentage != curr_percentage {
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

                if notified && status.percentage >= 10 {
                    notified = false;
                }

                if let Some(remaining) = remaining(Some(product_id)) {
                    handle
                        .tray_handle()
                        .get_item("remaining")
                        .set_title(&remaining)
                        .unwrap();
                }

                curr_percentage = status.percentage;
            }
        }
    });
}

fn remaining(product_id: Option<u16>) -> Option<String> {
    if let Some(product_id) = product_id {
        if let Some(status) = BatteryStatus::get(product_id) {
            if let Ok(remaining) = status.fmt_remaining() {
                return match remaining {
                    Some(r) => Some(format!("{} remaining", r)),
                    None => None,
                };
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
        menu = match razermacos::RazerDevices::new().all() {
            Some(devices) => {
                menu = menu
                    .add_item(CustomMenuItem::new("usage", "Usage Chart..."))
                    .add_item(CustomMenuItem::new("notify", "Test Notification"))
                    .add_native_item(SystemTrayMenuItem::Separator);

                for device in devices {
                    let id = format!("device_{}", device.product_id());
                    let mut item = CustomMenuItem::new(id, &device.name);
                    item.selected = product_id == device.product_id();
                    item.enabled = device.has_battery();
                    menu = menu.add_item(item);
                }

                menu.add_native_item(SystemTrayMenuItem::Separator)
                    .add_item(CustomMenuItem::new("devtools", "Open DevTools"))
            }
            None => no_devices_menu(&menu),
        };
    } else {
        menu = no_devices_menu(&menu);
    }

    menu.add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit"))
}

fn no_devices_menu(menu: &SystemTrayMenu) -> SystemTrayMenu {
    let menu = menu.clone();
    let mut item = CustomMenuItem::new("no_devices", "No devices found");
    item.enabled = false;
    menu.add_item(item)
}
