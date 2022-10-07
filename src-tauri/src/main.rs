#[cfg(any(target_os = "macos"))]
use razermacos::{devices::USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS, RazerDevices};
use std::{fmt, thread, time::Duration};
use tauri::{CustomMenuItem, Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu};

#[derive(Clone, Debug, serde::Serialize)]
struct BatteryStatus {
    pub percentage: u8,
    pub charging: bool,
}

impl fmt::Display for BatteryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let charging = if self.charging { " ⚡️" } else { "" };
        let icon = if self.percentage > 60 {
            "🔋"
        } else if self.percentage > 20 {
            "🪫"
        } else {
            "🔌"
        };
        write!(f, "{}{}%{}", icon, self.percentage, charging)
    }
}

fn main() {
    let status = if let Some(status) = get_status(USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS) {
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
                        let status = get_status(USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS);
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
        let status = get_status(USB_DEVICE_ID_RAZER_VIPER_ULTIMATE_WIRELESS);
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

fn get_status(device_id: u32) -> Option<BatteryStatus> {
    let mut devices = RazerDevices::all();
    let device = devices.find(device_id as u16);

    if let Some(device) = device {
        return Some(BatteryStatus {
            percentage: device.battery(),
            charging: device.is_charging(),
        });
    }

    None
}
