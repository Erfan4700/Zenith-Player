use tauri::{State, Window, Manager}; // اضافه کردن Manager برای دسترسی به state
use std::sync::Mutex;

pub struct PinState {
    pub mode: Mutex<String>,
    pub is_playing: Mutex<bool>,
}

// تعریف یک استیت جدید برای ذخیره مسیر فایل اولیه
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

// دستور جدید برای تحویل مسیر فایل به فرانت‌اند
#[tauri::command]
fn get_startup_file(state: State<'_, StartupState>) -> Option<String> {
    let mut path_lock = state.file_path.lock().unwrap();
    path_lock.take() // بازگرداندن مقدار و خالی کردن آن برای جلوگیری از لود مجدد
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        // مدیریت استیت‌ها
        .manage(PinState {
            mode: Mutex::new("off".to_string()),
            is_playing: Mutex::new(false),
        })
        .manage(StartupState {
            file_path: Mutex::new(None),
        })
        // بررسی آرگومان‌های خط فرمان هنگام لود اولیه
        .setup(|app| {
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 {
                let potential_path = &args[1];
                // بررسی وجود داشتن فایل در سیستم کاربر
                if std::path::Path::new(potential_path).exists() {
                    let state = app.state::<StartupState>();
                    let mut path_lock = state.file_path.lock().unwrap();
                    *path_lock = Some(potential_path.clone());
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            shadow_pin_control,
            report_video_status,
            perform_window_action,
            get_startup_file // ثبت دستور جدید
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}