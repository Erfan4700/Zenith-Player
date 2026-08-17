// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use tauri::{Emitter, Manager, State, Window};
use tauri_plugin_clipboard_manager::ClipboardExt;

// ساختار داده برای وضعیت پونز (Always on Top)
pub struct PinState {
    pub mode: Mutex<String>,
    pub is_playing: Mutex<bool>,
}

fn apply_pin_logic(window: &Window, mode: &str, is_playing: bool) -> Result<bool, String> {
    let should_pin = match mode {
        "always" => true,
        "off" => false,
        "playing" => is_playing,
        "paused" => !is_playing,
        _ => false,
    };
    window.set_always_on_top(should_pin).map_err(|e| e.to_string())?;
    Ok(should_pin)
}

// ۱. مدیریت دکمه‌های ویندوز (Minimize, Maximize, Close)
#[tauri::command]
fn perform_window_action(window: Window, action: String) {
    match action.as_str() {
        "minimize" => { let _ = window.minimize(); }
        "maximize" => {
            if let Ok(is_max) = window.is_maximized() {
                if is_max { let _ = window.unmaximize(); } else { let _ = window.maximize(); }
            }
        }
        "close" => { let _ = window.close(); }
        _ => {}
    }
}

// ۲. مدیریت پونز
#[tauri::command]
fn shadow_pin_control(window: Window, mode: String, state: State<'_, PinState>) -> Result<String, String> {
    let mut current_mode = state.mode.lock().unwrap();
    let current_playing = state.is_playing.lock().unwrap();
    *current_mode = mode.clone();
    let pinned = apply_pin_logic(&window, &current_mode, *current_playing)?;
    let status_text = if pinned { "فعال" } else { "غیرفعال" };
    Ok(format!("وضعیت پونز به {} تغییر یافت (پونز: {})", *current_mode, status_text))
}

#[tauri::command]
fn report_video_status(window: Window, is_playing: bool, state: State<'_, PinState>) -> Result<(), String> {
    let current_mode = state.mode.lock().unwrap();
    let mut current_playing = state.is_playing.lock().unwrap();
    *current_playing = is_playing;
    apply_pin_logic(&window, &current_mode, *current_playing)?;
    Ok(())
}

// ۳. کپی متن و تصویر در کلیپ‌بورد
#[tauri::command]
fn copy_text_to_clipboard(app: tauri::AppHandle, text: String) -> Result<(), String> {
    app.clipboard().write_text(text).map_err(|e| format!("خطا در کلیپ‌بورد: {}", e))?;
    Ok(())
}

#[tauri::command]
fn copy_image_to_clipboard(app: tauri::AppHandle, png_bytes: Vec<u8>) -> Result<(), String> {
    let decoded = image::load_from_memory(&png_bytes)
        .map_err(|e| format!("خطا در رمزگشایی تصویر: {}", e))?
        .to_rgba8();

    let (width, height) = decoded.dimensions();
    let rgba_raw = decoded.into_raw();
    let img = tauri::image::Image::new_owned(rgba_raw, width, height);
    
    app.clipboard().write_image(&img)
        .map_err(|e| format!("خطا در کلیپ‌بورد ویندوز: {}", e))?;
    
    Ok(())
}

// ۴. مدیریت آرگومان‌های استارت (فقط یک‌بار خوانده می‌شود تا در F5 تکرار نشود)
static ARGS_CONSUMED: AtomicBool = AtomicBool::new(false);

#[tauri::command]
fn get_cli_args() -> Vec<String> {
    if !ARGS_CONSUMED.swap(true, Ordering::SeqCst) {
        std::env::args().collect()
    } else {
        Vec::new()
    }
}

// تابع کمکی تشخیص پسوند ویدیو
fn is_video_path(path: &str) -> bool {
    let lower = path.to_lowercase();
    let media_exts = [
        // ویدیوها
        ".mp4", ".mkv", ".avi", ".mov", ".webm", ".ts", ".flv", ".wmv", ".m4v",
        // موزیک‌ها و فایل‌های صوتی 🎵
        ".mp3", ".wav", ".flac", ".aac", ".ogg", ".m4a", ".wma", ".opus", ".alac"
    ];
    media_exts.iter().any(|ext| lower.ends_with(ext))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        // دریافت فایل جدید زمانی که برنامه از قبل باز است
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            for arg in argv.iter().skip(1) {
                if is_video_path(arg) {
                    let _ = app.emit("open-video-file", arg.clone());
                    break;
                }
            }
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        .manage(PinState {
            mode: Mutex::new("off".to_string()),
            is_playing: Mutex::new(false),
        })
        .invoke_handler(tauri::generate_handler![
            perform_window_action,
            shadow_pin_control,
            report_video_status,
            copy_text_to_clipboard,
            copy_image_to_clipboard,
            get_cli_args
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}