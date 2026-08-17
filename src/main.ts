// ۱. دریافت ابزارهای ارتباطی و مدیریت پنجره از تائوری نسخه ۲ به صورت کاملاً تایپ‌آمن
const { invoke } = (window as any).__TAURI__.core;
// دریافت پنجره جاری از Tauri v2
const { getCurrentWindow } = (window as any).__TAURI__.window;
const appWindow = getCurrentWindow();

// تابع به‌روزرسانی آیکون Maximize/Restore
async function updateMaximizeIcon() {
    const isMaximized = await appWindow.isMaximized();
    const maximizeIcon = document.getElementById('icon-maximize');
    const restoreIcon = document.getElementById('icon-restore');
    if (maximizeIcon && restoreIcon) {
        if (isMaximized) {
            maximizeIcon.style.display = 'none';
            restoreIcon.style.display = 'inline';
        } else {
            maximizeIcon.style.display = 'inline';
            restoreIcon.style.display = 'none';
        }
    }
}

// تابع یکپارچه برای مدیریت کلیک روی دکمه Maximize/Restore
async function toggleMaximize() {
    const isMaximized = await appWindow.isMaximized();
    if (isMaximized) {
        await appWindow.unmaximize();
    } else {
        await appWindow.maximize();
    }
    await updateMaximizeIcon(); // به‌روزرسانی آیکون بعد از تغییر
}

//================================================================
// متغیری برای ذخیره وضعیت ماکسیمایز قبل از ورود به فول‌اسکرین
let wasMaximizedBeforeFullscreen = false;
async function toggleFullscreenSmart() {
    // اگر در حال حاضر فول‌اسکرین هستیم، فقط از آن خارج شو
    if (document.fullscreenElement) {
        await document.exitFullscreen();
        return;
    }

    const isMaximized = await appWindow.isMaximized();
    
    if (isMaximized) {
        wasMaximizedBeforeFullscreen = true; // ذخیره وضعیت برای بازگشت بعدی
        await invokeWindowAction('maximize'); // خروج از ماکسیمایز
        
        try {
            await document.body.requestFullscreen();
        } catch (err) {
            console.error("Error entering fullscreen:", err);
        }
    } else {
        wasMaximizedBeforeFullscreen = false; // وضعیت قبلی ماکسیمایز نبوده است
        try {
            await document.body.requestFullscreen();
        } catch (err) {
            console.error("Error entering fullscreen:", err);
        }
    }
}

// اکسپوز کردن مجدد برای دسترسی HTML
(window as any).toggleFullscreenSmart = toggleFullscreenSmart;
//============================================

// اکسپوز کردن توابع برای استفاده در HTML
(window as any).toggleMaximize = toggleMaximize;
(window as any).updateMaximizeIcon = updateMaximizeIcon;

// گوش دادن به رویداد تغییر اندازه پنجره (برای مواردی که کاربر از سیستم عامل ماکسیمایز می‌کند)
appWindow.onResized(async () => {
    await updateMaximizeIcon();
});

// آرایه وضعیت‌های ۴ حالته پونز
const pinModes = ["off", "always", "playing", "paused"];
let currentPinIndex = 0;

// ۲. مدیریت کلیدهای میانبر (Hotkeys) بدون تداخل
document.addEventListener('keydown', async (event: KeyboardEvent) => {
    const key = event.key.toUpperCase();
    
    // حل خطای تایپ‌اسکریپت با بررسی دقیق نوع target
    const target = event.target as HTMLElement | null;
    if (target && (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA')) {
        return;
    }

    switch(key) {
        case 'M': // Mute / Unmute
            event.preventDefault();
            await invoke('toggle_mute');
            break;
            
        
        // case 'F11': // Fullscreen
        //     event.preventDefault();
        //     toggleFullscreenSmart();
        //     break;

        case 'F': // Next Frame
            event.preventDefault();
            await invoke('next_frame');
            break;

        case 'E': // Shift + E برای اکولایزر
            if (event.shiftKey) {
                event.preventDefault();
                toggleEqualizerPanel();
            }
            break;

        case '9': // Zoom In
            await invoke('adjust_zoom', { value: 0.1 });
            break;
        case '1': // Zoom Out
            await invoke('adjust_zoom', { value: -0.1 });
            break;
        case '5': // Reset Zoom
            await invoke('adjust_zoom', { value: 0.0 });
            break;
    }
});

// ۳. تابع اصلی چرخش وضعیت پونز (این همان تابعی است که کار نمی‌کرد)
async function cyclePinMode() {
    currentPinIndex = (currentPinIndex + 1) % pinModes.length;
    const newMode = pinModes[currentPinIndex];
    
    try {
        // نام کامند باید دقیقاً با نام تابع در Rust (shadow_pin_control) یکی باشد
        const response = await invoke('shadow_pin_control', { mode: newMode });
        console.log(response);
        updatePinIconUI(newMode);
    } catch (err) {
        console.error("خطا در ارسال وضعیت پونز به راست:", err);
    }
}

// ۴. تابع آپدیت گرافیکی دکمه پونز در نوار بالایی
function updatePinIconUI(mode: string) {
    const pinBtn = document.getElementById('pin-button');
    if (!pinBtn) return;
    
    if (mode === 'off') {
        pinBtn.style.color = 'rgba(255, 255, 255, 0.55)';
        pinBtn.style.background = 'transparent';
        (pinBtn as any).style.filter = 'none';
    } else if (mode === 'always') {
        pinBtn.style.color = '#ef4444'; // قرمز درخشان
        (pinBtn as any).style.filter = 'drop-shadow(0 0 5px rgba(239, 68, 68, 0.6))';
    } else if (mode === 'playing') {
        pinBtn.style.color = '#38bdf8'; // آبی آسمانی
        (pinBtn as any).style.filter = 'drop-shadow(0 0 5px rgba(56, 189, 248, 0.6))';
    } else if (mode === 'paused') {
        pinBtn.style.color = '#fbbf24'; // زرد کهربایی
        (pinBtn as any).style.filter = 'drop-shadow(0 0 5px rgba(251, 191, 36, 0.6))';
    }
}

// ۵. نمایش پنل اکولایزر
function toggleEqualizerPanel() {
    const eqPanel = document.getElementById('equalizer-panel');
    if (eqPanel) {
        eqPanel.classList.toggle('hidden');
    }
}

// متصل کردن عملکردهای دکمه‌های پنجره بعد از لود کامل DOM
document.addEventListener('DOMContentLoaded', () => {
    // ۱. متصل کردن عملکردهای ویندوز
    const minBtn = document.getElementById('win-minimize');
    const maxBtn = document.getElementById('win-maximize');
    const closeBtn = document.getElementById('win-close');

    if (minBtn) minBtn.addEventListener('click', () => appWindow.minimize());
    if (maxBtn) maxBtn.addEventListener('click', () => appWindow.toggleMaximize());
    if (closeBtn) closeBtn.addEventListener('click', () => appWindow.close());

    // ۲. 🎥 هوشمندسازی پونز: اتصال مستقیم به تگ ویدیو پلیر شما
    // نکته: اگر تگ ویدیو شما کلاس یا آیدی خاصی دارد (مثلاً id="main-video")، آن را جایگزین 'video' کن
    const videoElement = document.querySelector('video');
    
    if (videoElement) {
        // وقتی ویدیو پلی می‌شود
        videoElement.addEventListener('play', async () => {
            console.log("ویدیو به حالت پخش درآمد -> گزارش به راست");
            await invoke('report_video_status', { isPlaying: true });
        });
        
        // وقتی ویدیو پاز (متوقف) می‌شود
        videoElement.addEventListener('pause', async () => {
            console.log("ویدیو متوقف شد -> گزارش به راست");
            await invoke('report_video_status', { isPlaying: false });
        });
    } else {
        console.warn("تگ <video> در صفحه پیدا نشد! مطمئن شو که ویدیو لود شده است.");
    }
    
});

// 📌 فوق‌العاده مهم: اکسپوز کردن تابع برای اینکه فایل HTML بتواند آن را ببیند
(window as any).cyclePinMode = cyclePinMode;


// این تابع را به انتهای فایل src/main.ts اضافه کن
async function invokeWindowAction(action: string) {
    try {
        await invoke('perform_window_action', { action: action });
    } catch (err) {
        console.error("Error executing window action:", err);
    }
}

// اکسپوز کردن تابع برای تگ‌های HTML
(window as any).invokeWindowAction = invokeWindowAction;


// این تابع وظیفه آپدیت کردن متن وسط تایتل‌بار را دارد
function updateTitlebar(fileName: string) {
    const titleElement = document.getElementById('video-title');
    if (titleElement) {
        titleElement.textContent = fileName;
    }
}

// --- ترفند هوشمندانه: ذخیره اتوماتیک نام فایل‌ها بدون نیاز به تغییر کدهای دیگر ---
const blobFileNames = new Map<string, string>();
const originalCreateObject = URL.createObjectURL;

URL.createObjectURL = function (obj: Blob | MediaSource): string {
    const url = originalCreateObject(obj);
    // اگر چیزی که در حال تبدیل شدن به ویدیو است یک فایل باشد، نام آن را ذخیره می‌کنیم
    if (obj instanceof File) {
        blobFileNames.set(url, obj.name);
    }
    return url;
};
// -----------------------------------------------------------------------------

document.addEventListener('DOMContentLoaded', () => {
    const videoElement = document.querySelector('video') as HTMLVideoElement;
    
    if (videoElement) {
        // وقتی سورس ویدیو عوض می‌شود
        videoElement.addEventListener('loadedmetadata', () => {
            const videoSrc = videoElement.currentSrc;
            
            // ۱. اگر آدرس از نوع blob بود (فایل از سیستم کاربر انتخاب شده)
            if (videoSrc.startsWith('blob:') && blobFileNames.has(videoSrc)) {
                const realFileName = blobFileNames.get(videoSrc) as string;
                updateTitlebar(realFileName);
            } 
            // ۲. اگر آدرس معمولی یا اینترنتی بود (روش قبلی خودت)
            else {
                const fileName = videoSrc.split('/').pop()?.split('?')[0] || "Unknown Video";
                updateTitlebar(decodeURIComponent(fileName));
            }
        });
    }
    (async () => {
    try {
        const startupPath: string | null = await invoke('get_startup_file');
        if (startupPath) {
            const { convertFileSrc } = (window as any).__TAURI__.core;
            
            // تبدیل مسیر لوکال سیستم به آدرس قابل پخش در تگ ویدیو
            const assetUrl = convertFileSrc(startupPath);
            const fileName = startupPath.split(/[/\\]/).pop() || "Local Video";

            const videoElement = document.querySelector('video') as HTMLVideoElement;
            if (videoElement) {
                videoElement.src = assetUrl;
                videoElement.load();
                videoElement.play().catch(err => console.error("Playback startup video failed:", err));
                
                // به‌روزرسانی نام ویدیو در هدر (Titlebar)
                if ((window as any).updateTitlebar) {
                    (window as any).updateTitlebar(fileName);
                }
            }
        }
    } catch (err) {
        console.error("Error retrieving startup file path:", err);
    }
})();
});

// اکسپوز کردن تابع برای استفاده در بخش‌های دیگر برنامه
(window as any).updateTitlebar = updateTitlebar;



/////new
// گوش دادن به تغییرات وضعیت فول‌اسکرین مرورگر برای بازگشت به حالت قبلی
document.addEventListener('fullscreenchange', async () => {
    // اگر کاربر از حالت فول‌اسکرین خارج شد
    if (!document.fullscreenElement) {
        if (wasMaximizedBeforeFullscreen) {
            // برگشت به حالت ماکسیمایز سیستم‌عامل
            await invokeWindowAction('maximize');
            wasMaximizedBeforeFullscreen = false; // ریست کردن پرچم
        }
    }
});




// =================================================================
// 🎥 سیستم جامع لود و پخش فایل‌های سیستم (Drag & Drop + Open With)
// =================================================================

function playLocalPath(filePath: string) {
    if (!filePath) return;

    try {
        const { convertFileSrc } = (window as any).__TAURI__.core;
        
        // تبدیل مسیر واقعی سیستم‌عامل (مثلاً C:\Videos\movie.mp4) به آدرس قابل پخش
        const assetUrl = convertFileSrc(filePath);
        const fileName = filePath.split(/[/\\]/).pop() || "Local Video";

        const videoElement = document.querySelector('video') as HTMLVideoElement;
        if (videoElement) {
            videoElement.src = assetUrl;
            videoElement.load();
            videoElement.play().catch(err => console.error("Playback failed:", err));
            
            // به‌روزرسانی نام در Titlebar
            if ((window as any).updateTitlebar) {
                (window as any).updateTitlebar(fileName);
            }
        }
    } catch (err) {
        console.error("Error playing local file path:", err);
    }
}

// اکسپوز به سراسر برنامه
(window as any).playLocalPath = playLocalPath;

document.addEventListener('DOMContentLoaded', async () => {
    // ۱. بررسی فایل ارسالی هنگام اجرای اولیه برنامه (وقتی برنامه از ابتدا بسته بوده)
    try {
        const startupPath: string | null = await invoke('get_startup_file');
        if (startupPath) {
            playLocalPath(startupPath);
        }
    } catch (err) {
        console.error("Error getting startup file:", err);
    }

    // ۲. گوش دادن به فایلی که بعداً روی آیکون درگ می‌شود (وقتی برنامه باز است)
    const { listen } = (window as any).__TAURI__.event;
    if (listen) {
        await listen('open-file-from-system', (event: any) => {
            const filePath = event.payload as string;
            if (filePath) {
                playLocalPath(filePath);
            }
        });
    }

    // ۳. گوش دادن به درگ و دراپ مستقیم فایل‌ها داخل خود پنجره پلیر
    if (appWindow && appWindow.onDragDropEvent) {
        await appWindow.onDragDropEvent((event: any) => {
            if (event.payload && event.payload.type === 'drop') {
                const paths = event.payload.paths as string[];
                if (paths && paths.length > 0) {
                    playLocalPath(paths[0]);
                }
            }
        });
    }
});