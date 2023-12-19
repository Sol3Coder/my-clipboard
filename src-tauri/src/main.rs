// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::{DateTime, Local};
use clipboard_win::{formats, Clipboard, Getter, Setter};
use std::thread;
use std::time::Duration;
use tauri::Manager;
#[derive(Clone, serde::Serialize)]
struct ReadMsg {
    msg: String,
    msg_time: String,
    len: usize,
}
fn monite_clip<R: tauri::Runtime>(manager: &impl Manager<R>) {
    let _clip = Clipboard::new_attempts(10).expect("Open clipboard");

    let mut output = String::new();
    formats::Unicode
        .read_clipboard(&mut output)
        .expect("Read sample");
    println!("{}", output);
    //    ctx.set_contents("some string".to_owned()).unwrap();
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                // A loop that takes output from the async process and sends it
                // to the webview via a Tauri Event

                loop {
                    monite_clip(&app_handle);
                    thread::sleep(Duration::from_millis(1000));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
