// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    thread::{self, sleep},
    time::Duration,
};

use chrono::prelude::*;
use config::get_config;
use tauri::SystemTray;

mod config;

fn get_system_tray() -> SystemTray {
    let tray = SystemTray::new();

    tray
}

fn main() {
    let config = get_config();

    let target_dt = config
        .target_date
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Utc)
        .unwrap();
    let tray = get_system_tray();

    tauri::Builder::default()
        .system_tray(tray)
        .setup(move |app| {
            let systemtrayhandle = app.tray_handle();

            thread::spawn(move || loop {
                let now = Utc::now();

                let difference = target_dt - now;

                let weeks = difference.num_weeks();
                let days = difference.num_days();
                let days = days - (weeks * 7);

                let title = format!("{} {}w {}d", config.name, weeks, days);

                systemtrayhandle.set_title(&title).unwrap();

                sleep(Duration::from_secs(60));
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
