# 🌟 Zenith Player

<div align="center">
  <img src="src-tauri/icons/Square310x310Logo.png" alt="Zenith Player Logo" width="150"/>
  
  <p><b>A modern, lightning-fast, and feature-rich desktop media player with a stunning Glassmorphism UI</b></p>
  
  [![Tauri](https://img.shields.io/badge/Built%20with-Tauri%20v2-24C8D6?style=flat-square&logo=tauri&logoColor=white)](https://tauri.app/)
  [![JavaScript](https://img.shields.io/badge/JavaScript-F7DF1E?style=flat-square&logo=javascript&logoColor=black)]()
  [![HTML/CSS](https://img.shields.io/badge/HTML5%20&%20CSS3-E34F26?style=flat-square&logo=html5&logoColor=white)]()
  [![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)]()
</div>

<br/>

**Zenith Player** is an advanced desktop media player built with modern web technologies (HTML/CSS/JS) and the ultra-lightweight **Tauri v2** framework (Rust). Delivering not only a breathtaking frosted-glass aesthetic, it provides rich features such as **Music & Video Playback**, **Interactive Language-Learning Subtitles (Word-by-word Click & Copy)**, an **Interactive Screenshot & Annotation Studio**, **Pan & Zoom controls**, and an **Advanced Bookmarking System**.

> 🌐 **Language / زبان:** [فارسی (Persian)](./README_FA.md)

---

## 💡 Behind the Project

> 🤖 **Fun Fact / AI-Powered Journey:**  
> This entire application was **built 100% using Artificial Intelligence**, continuously directed, architected, and prompt-engineered by me over **several months**. It took hundreds of iterations, deep-dive debugging sessions, and resolving **hundreds of complex edge cases** to bring Zenith Player to this level of polish and performance. It stands as a testament to what human-guided AI development can achieve today!

---

## ✨ Key Features

### 🎨 UI & UX Design
*   **Glassmorphism Aesthetic:** Premium frosted-glass look inspired by modern design standards (Video.js v10 style).
*   **Windows 11 Custom Titlebar:** Seamless window controls (Minimize, Maximize/Restore, Close) and an **Always-on-Top (Pin)** toggle.
*   **Dark / Light Theme:** Instant dynamic theme switcher with harmonized color accents.
*   **Dynamic Ambient Orbs:** Fluid, glowing background orbs that bring depth to playback.
*   **Hardware FastSeek Thumbnail Preview:** Near-instant hover frame preview on the timeline with virtually zero RAM footprint.

### 🎬 Media Playback & Control (Video & Audio)
*   **Universal Media Support:** Plays video formats (MP4, MKV, AVI, MOV, WEBM, TS, FLV) and music formats (MP3, FLAC, WAV, AAC, M4A, OGG).
*   **Deep Windows Integration:** Open files directly via Double-Click, Context Menu (Open with), CLI args, or Drag & Drop onto the app icon.
*   **Smart Speed Control:** Smooth playback speed adjustments from **0.1x** up to **10.0x**.
*   **Pan & Zoom:** Freeform zooming and panning across the video frame via Numpad keys or Ctrl + Mouse Wheel.
*   **Frame Stepping:** Step forward frame-by-frame with precision using the `F` key.
*   **Unified Resume Playback:** Automatically remembers exact timestamps across all open methods without overwrite bugs.
*   **Picture-in-Picture (PiP):** Multitask effortlessly with a floating video overlay.

### 📝 Interactive Subtitle & Learning Engine
*   **SRT, VTT, and ASS Support:** Blazing-fast subtitle parsing powered by dedicated Web Workers.
*   **Interactive Word Capsules:** Subtle frosted-glass hover capsules around individual words without disrupting natural line spacing.
*   **Click-to-Copy for Language Learners:** Single-click on any word to copy it instantly to the clipboard, or press `D` / `Shift + D` to copy entire sentences with glowing emerald feedback.
*   **Dual Subtitles:** Display two independent subtitle tracks (e.g., Target Language & Native Language) simultaneously.
*   **Complete Styling:** Freely customize font size, color, background (frosted, solid, black, blue), and direction (RTL/LTR).

### 📸 Screenshot & Drawing Studio
*   **Frame Capture with Subtitles:** High-resolution frame grabs including active burned-in subtitles.
*   **Interactive Cropping:** Crop and frame captures in real-time before exporting.
*   **Full Drawing Studio:** Pencil, Eraser, Stroke Eraser, Line, Arrow, Rectangle, Circle, and Triangle tools with custom color palettes and stroke weights.
*   **Full Undo / Redo:** Complete state history via `Ctrl+Z` and `Ctrl+Y`.
*   **Native Clipboard Export:** Copy PNGs directly into the Windows clipboard or download as high-res files.

### 🔖 Bookmarks, Playlist & History
*   **Interactive Bookmarks:** Pin timestamps with custom names, rendered as golden markers on the progress bar.
*   **Dynamic Playlist:** Resizable playlist panel with automatic video matching and active state highlights.
*   **Online Stream History:** Automatically stores played URLs for one-click replaying.

---

## ⌨️ Keyboard Shortcuts

### Playback Controls
| Shortcut | Action |
| :--- | :--- |
| `Space` | Play / Pause |
| `F5` | **Full Stop Playback** and return to start |
| `Arrow Right` | Seek forward 5 seconds |
| `Arrow Left` | Seek backward 5 seconds |
| `Arrow Up` | Increase volume |
| `Arrow Down` | Decrease volume |
| `C` | Increase playback speed (+0.1) |
| `X` | Decrease playback speed (-0.1) |
| `Z` | Toggle between 1.0x and previous speed |
| `B` | Play next item in playlist |
| `N` | Play previous item in playlist |

### Video & UI Controls
| Shortcut | Action |
| :--- | :--- |
| `F11` | Toggle Fullscreen |
| `P` | Open / Close Bookmarks panel |
| `Ctrl + P` | Toggle Picture-in-Picture (PiP) |
| `Numpad 8 / 2 / 4 / 6` | Scale video along X and Y axes |
| `Ctrl + Numpad 8 / 2 / 4 / 6` | Pan video frame (Up, Down, Left, Right) |
| `Numpad 9 / 1` | Zoom In / Zoom Out video frame |
| `Numpad 5` | Reset video Pan & Zoom |
| `Ctrl + Alt + Arrow Up / Down` | Adjust bottom control bar margin |
| `Ctrl + Alt + + / - / =` | Scale bottom control bar |
| `Ctrl + Shift + + / - / =` | Scale settings panel |
| `<` and `>` (Comma & Period) | **Reset all UI panel scales to default** |

### Subtitle & Learning Controls
| Shortcut | Action |
| :--- | :--- |
| `D` | **Copy Primary Subtitle sentence** + Emerald flash effect |
| `Shift + D` | **Copy Secondary Subtitle sentence** + Emerald flash effect |
| `Click` on word | **Copy individual word** to clipboard |
| `Shift + Click` | Copy full subtitle sentence |
| `=` / `+` | Increase font size of **Primary Subtitle (Bottom)** |
| `-` | Decrease font size of **Primary Subtitle (Bottom)** |
| `Shift + =` / `Shift + +` | Increase font size of **Secondary Subtitle (Top)** |
| `Shift + -` | Decrease font size of **Secondary Subtitle (Top)** |
| `Ctrl + Arrow Keys` | Reposition **Primary Subtitle** |
| `Ctrl + Shift + Arrow Keys`| Reposition **Secondary Subtitle** |
| `[` and `]` | **Reset position and font size for both subtitles** |

### Bookmarks
| Shortcut | Action |
| :--- | :--- |
| `K` | Add bookmark at current position (with name dialog) |
| `L` | Remove bookmark at current position |

### Screenshot & Drawing Editor
| Shortcut | Action |
| :--- | :--- |
| `S` | Take screenshot and open editor |
| `Escape` | Exit drawing / Exit crop / Close modal |
| `Enter` | Confirm and apply drawing or crop |
| `Ctrl + Z` | Undo last stroke |
| `Ctrl + Y` | Redo last stroke |

---

## 🚀 Installation & Setup

1. [Node.js](https://nodejs.org/) (v18+)
2. [Rust](https://www.rust-lang.org/)
3. Platform build tools (C++ Build Tools on Windows)

### Development Setup:
```bash
# 1. Clone repository
git clone https://github.com/your-username/zenith-player.git
cd zenith-player

# 2. Install dependencies
npm install

# 3. Run in development mode
npm run tauri dev

# 4. Build standalone production installer
npm run tauri build