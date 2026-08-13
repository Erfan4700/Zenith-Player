# 🌟 Zenith Player

<div align="center">
  <img src="src-tauri/icons/Square310x310Logo.png" alt="Zenith Player Logo" width="150"/>
  
  <p><b>A modern, fast, and feature-rich desktop video player with a stunning Glassmorphism UI</b></p>
  
  [![Tauri](https://img.shields.io/badge/Built%20with-Tauri-24C8D6?style=flat-square&logo=tauri&logoColor=white)](https://tauri.app/)
  [![JavaScript](https://img.shields.io/badge/JavaScript-F7DF1E?style=flat-square&logo=javascript&logoColor=black)]()
  [![HTML/CSS](https://img.shields.io/badge/HTML5%20&%20CSS3-E34F26?style=flat-square&logo=html5&logoColor=white)]()
  [![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)]()
</div>

<br/>

**Zenith Player** is an advanced desktop video player built with web technologies (HTML/CSS/JS) and the powerful **Tauri** framework (Rust). It delivers not only a visually stunning experience but also powerful tools such as **Dual Subtitles**, an **Interactive Screenshot & Drawing Editor**, **Pan & Zoom controls**, and an **Advanced Bookmarking System**.

> 🌐 **Language / زبان:** [فارسی (Persian)](./README_FA.md)

---

## 💡 Behind the Project

> 🤖 **Fun Fact / AI-Powered Journey:**
> This entire application was **built 100% using Artificial Intelligence**, continuously directed, architected, and prompt-engineered by me over **several months**. It took hundreds of iterations, deep-dive debugging sessions, and resolving **hundreds of complex bugs** to bring Zenith Player to this level of polish and feature density. It stands as a testament to what human-guided AI development can achieve today!

---

## ✨ Key Features

### 🎨 UI & UX Design
*   **Glassmorphism Design:** Stunning frosted-glass aesthetic inspired by modern UI standards (Video.js v10 style).
*   **Dark / Light Mode:** Instant theme switching with smart adaptive color palettes.
*   **Dynamic Ambient Orbs:** Animated background glow effects that enhance the viewing experience.
*   **Floating Preview:** Thumbnail hover preview with timestamp display over the progress bar.

### 🎬 Playback & Video Control
*   **Local & Online Playback:** Play local files via Drag & Drop or stream directly via URL.
*   **Smart Speed Control:** Fine-tune playback speed up to **10x**.
*   **Pan & Zoom:** Zoom in/out and pan across the video frame using Numpad shortcuts or mouse scroll.
*   **Color Filters:** Applied Luma and Invert color filters for video and screenshots.
*   **Resume Playback:** Automatically remembers your last playback position for seamless resuming.
*   **Picture-in-Picture (PiP):** Watch videos in a compact floating window while multitasking.

### 📝 Advanced Subtitle System
*   **SRT, VTT, and ASS Support:** Blazing-fast subtitle parsing powered by Web Workers.
*   **Dual Subtitles:** Display two subtitle tracks simultaneously (e.g., English and native language) at the top and bottom of the screen.
*   **Full Customization:** Independently adjust font size, colors, backgrounds (glass, solid black, blue, etc.), and text direction (RTL/LTR) for both tracks.
*   **Smart Match:** Automatically detects and loads subtitle files matching the video name.

### 📸 Screenshot & Drawing Tools
*   **High-Quality Screenshots:** Captures frames with active subtitles burnt-in onto the image.
*   **Live Crop Tool:** Interactive image cropping prior to saving.
*   **Annotation & Drawing:** Built-in drawing studio featuring Pencil, Eraser, Line, Arrow, Rectangle, Circle, and Triangle tools with custom colors and stroke widths.
*   **Undo / Redo History:** Full state management for annotations.
*   **Quick Export:** Copy directly to Clipboard or save as a high-res PNG file.

### 🔖 Bookmarks, History & Playlist
*   **Interactive Bookmarks:** Bookmark specific timestamps with custom labels, visually highlighted on the progress bar.
*   **Resizable Playlist:** Manage video queues with a customizable side panel width.
*   **Online Stream History:** Automatically stores played URLs for quick future access.

---

## ⌨️ Keyboard Shortcuts

Zenith Player is fully controllable via keyboard for power users.

### Playback Controls
| Shortcut | Action |
| :--- | :--- |
| `Space` | Play / Pause |
| `Arrow Right` | Seek forward 5 seconds |
| `Arrow Left` | Seek backward 5 seconds |
| `Arrow Up` | Increase volume |
| `Arrow Down` | Decrease volume |
| `C` | Increase speed (+0.1) |
| `X` | Decrease speed (-0.1) |
| `Y` | Reset speed to 1.0x (or toggle back to previous) |
| `B` | Play next item in playlist |
| `N` | Play previous item in playlist |

### Video & UI Controls
| Shortcut | Action |
| :--- | :--- |
| `F11` | Toggle Fullscreen |
| `P` | Open / Close Bookmarks Panel |
| `Ctrl + P` | Toggle Picture-in-Picture (PiP) |
| `Numpad 8 / 2 / 4 / 6` | Scale / Zoom video along X and Y axes |
| `Ctrl + Numpad 8/2/4/6` | Pan video frame (Up, Down, Left, Right) |
| `Numpad 5` | Reset video Pan & Zoom |
| `Ctrl + Alt + Arrow Up/Down` | Adjust bottom control bar margin |
| `Ctrl + Alt + + / - / =` | Zoom In / Out bottom control bar |
| `Ctrl + Shift + + / - / =` | Zoom In / Out settings panel |
| `<` and `>` (Comma & Period) | **Reset all UI panel scales to default** |

### Subtitle Controls
| Shortcut | Action |
| :--- | :--- |
| `=` / `+` | Increase font size of **Primary Subtitle (Bottom)** |
| `-` | Decrease font size of **Primary Subtitle (Bottom)** |
| `Shift + =` / `Shift + +` | Increase font size of **Secondary Subtitle (Top)** |
| `Shift + -` | Decrease font size of **Secondary Subtitle (Top)** |
| `Ctrl + Arrow Keys` | Reposition **Primary Subtitle** |
| `Ctrl + Shift + Arrow Keys`| Reposition **Secondary Subtitle** |
| `[` and `]` | **Reset position and size for both subtitles** |

### Bookmarks
| Shortcut | Action |
| :--- | :--- |
| `K` | Add bookmark at current time (opens prompt) |
| `L` | Delete bookmark at current time |

### Screenshot & Drawing Editor
| Shortcut | Action |
| :--- | :--- |
| `S` | Take screenshot and open Drawing Editor |
| `Escape` | Exit drawing / Exit crop mode / Close modal |
| `Enter` | Confirm and apply drawing or crop |
| `Ctrl + Z` | Undo last stroke |
| `Ctrl + Y` or `Ctrl + Shift + Z` | Redo last stroke |

---

## 🚀 Installation & Setup

Since this project is built with **Tauri**, you need the following prerequisites installed on your system:
1. [Node.js](https://nodejs.org/) (v16+)
2. [Rust](https://www.rust-lang.org/)
3. Platform-specific build tools (C++ Build tools on Windows, `webkit2gtk` on Linux, etc.)

### Development Setup:
```bash
# 1. Clone the repository
git clone https://github.com/your-username/zenith-player.git
cd zenith-player

# 2. Install dependencies
npm install

# 3. Run in development mode
npm run tauri dev

# 4. Build for production
npm run tauri build
