use tauri::{State, Window, Manager, Emitter};
use std::sync::Mutex;

use tauri_plugin_clipboard_manager::ClipboardExt;


pub struct PinState {
    pub mode: Mutex<String>,
    pub is_playing: Mutex<bool>,
}

pub struct StartupState {
    pub file_path: Mutex<Option<String>>,
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

#[tauri::command]
fn get_startup_file(state: State<'_, StartupState>) -> Option<String> {
    let mut path_lock = state.file_path.lock().unwrap();
    path_lock.take()
}

// تابع کمکی برای پیدا کردن اولین مسیر فایل معتبر در میان آرگومان‌ها
fn find_valid_file_path(args: &[String]) -> Option<String> {
    args.iter().skip(1).find_map(|arg| {
        let p = std::path::Path::new(arg);
        if p.is_file() {
            Some(arg.clone())
        } else {
            None
        }
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        // 👈 پلاگین مدیریت ارسال فایل هنگام باز بودن برنامه
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            if let Some(file_path) = find_valid_file_path(&args) {
                // ارسال مسیر فایل به فرانت‌اند
                let _ = app.emit("open-file-from-system", file_path);
            }
            // فوکوس روی پنجره موجود
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
        .manage(StartupState {
            file_path: Mutex::new(None),
        })
        // 👈 خواندن فایل وقتی برنامه بسته بوده و تازه اجرا شده
        .setup(|app| {
            let args: Vec<String> = std::env::args().collect();
            if let Some(file_path) = find_valid_file_path(&args) {
                let state = app.state::<StartupState>();
                let mut path_lock = state.file_path.lock().unwrap();
                *path_lock = Some(file_path);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            shadow_pin_control,
            report_video_status,
            perform_window_action,
            get_startup_file,
            copy_image_to_clipboard,
            copy_text_to_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn copy_image_to_clipboard(app: tauri::AppHandle, png_bytes: Vec<u8>) -> Result<(), String> {
    // ۱. رمزگشایی بایت‌های PNG به داده‌های خام RGBA
    let decoded = image::load_from_memory(&png_bytes)
        .map_err(|e| format!("خطا در رمزگشایی تصویر: {}", e))?
        .to_rgba8();

    let (width, height) = decoded.dimensions();
    let rgba_raw = decoded.into_raw();

    // ۲. ساخت تصویر تائوری با استفاده از متد new_owned
    let img = tauri::image::Image::new_owned(rgba_raw, width, height);
    
    // ۳. کپی مستقیم در کلیپ‌بورد سیستم‌عامل
    app.clipboard().write_image(&img)
        .map_err(|e| format!("خطا در کلیپ‌بورد ویندوز: {}", e))?;
    
    Ok(())
}


#[tauri::command]
fn copy_text_to_clipboard(app: tauri::AppHandle, text: String) -> Result<(), String> {
    app.clipboard().write_text(text).map_err(|e| format!("خطا در کلیپ‌بورد: {}", e))?;
    Ok(())
}







