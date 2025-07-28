<div align="center">

# QuickNote for macOS

[繁體中文](./README.zh-TW.md)

![License](https://img.shields.io/badge/license-CC--BY--NC--SA--4.0-blue.svg)
![Version](https://img.shields.io/badge/version-0.1.0-brightgreen.svg)
![Tauri](https://img.shields.io/badge/Tauri-24C8DB?logo=tauri&logoColor=white)
![Vue.js](https://img.shields.io/badge/Vue.js-4FC08D?logo=vue.js&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-DEA584?logo=rust&logoColor=white)

**A simple, fast, and lightweight note-taking application for macOS, designed for quick access and ease of use.**

</div>

---

### ✨ **QuickNote in Action**

**Light Theme**
![QuickNote Dark Theme](https://github.com/user-attachments/assets/8eda5e60-601d-467c-8861-0eb536d9e58c)

**Dark Theme**
![QuickNote Light Theme](https://github.com/user-attachments/assets/355d944b-36b7-40d5-9959-40ca30dc9370)

**Settings Panel (light)**
![QuickNote Settings Dark](https://github.com/user-attachments/assets/27a36411-048e-4f71-b339-1c85b1eda738)

**Settings Panel (Dark)**
![QuickNote Settings Light](https://github.com/user-attachments/assets/6c5bb556-3701-46e8-8cc3-5a476a1accf1)

---

## 🚀 Key Features

QuickNote isn't just another text editor. It's meticulously crafted to be an extension of your mind, ready to capture thoughts frictionlessly and boost your productivity.

### Core Experience & Performance
*   ⚡️ **Instant Global Access:** Summon a new note or toggle your workspace from anywhere in your OS with customizable global hotkeys. Capture ideas the moment they strike, without ever leaving your current context.
*   💨 **Native Performance, Feather-light Footprint:** Built with a Rust backend and a Tauri core, QuickNote launches instantly and runs with minimal CPU and memory usage. Experience native speed and responsiveness that web-based apps can't match.
*   🧠 **Focus-Aware Activation:** When you summon QuickNote via a shortcut, it intelligently brings itself to the foreground and stays on top temporarily, ensuring you can start typing immediately without the window disappearing.

### Intuitive & Modern Interface
*   🗂️ **Advanced Tab Management:** Organize your thoughts with a powerful tabbed interface. Effortlessly **drag-and-drop** to reorder tabs, **double-click to rename**, and see unsaved changes at a glance with a visual indicator (`•`).
*   ✍️ **Minimalist yet Powerful Editor:** A clean, distraction-free writing environment featuring essential tools like **line numbers** and proper **tab indentation** support, helping you focus on what matters: your content.
*   🌓 **Dynamic Theming:** Seamlessly switch between a beautiful **Light** and a comfortable **Dark** theme. Your choice is saved and applied instantly, adapting the entire UI to your preference.
*   🖼️ **Sleek Frameless Design (macOS):** Enjoy a modern, unified aesthetic on macOS with a custom frameless window and fully functional, integrated **"traffic light"** controls for a truly native feel.
*   📂 **Drag & Drop to Open:** Simply drag files from your desktop or file explorer and drop them onto the QuickNote window to open them in new tabs instantly.

### Powerful Customization
*   ⌨️ **Fully Configurable Global Hotkeys:** Take full control of your workflow by customizing the keyboard shortcuts for creating a new note, closing a tab, and showing/hiding the application window.
*   ⚙️ **Workflow Preferences:** Streamline your process by setting a **default save path** for new notes and choosing a **default file format** (`.txt`, `.md`, etc.), minimizing repetitive actions.
*   🌐 **Multi-Language Interface:** Switch between **English** and **Traditional Chinese** on-the-fly. The entire UI, from settings to status bar, will update instantly.

### Advanced Workflow & Productivity
*   💾 **Smart Save System:** QuickNote is smart about saving. It uses `Save` for existing files and automatically prompts `Save As...` for new, untitled notes, ensuring you never accidentally overwrite the wrong file.
*   🛡️ **Unsaved Changes Protection:** Never lose your work again. QuickNote will warn you and ask for confirmation before closing a tab with unsaved changes.
*   📦 **Intelligent System Tray Menu:** Access core functions without even opening the window. The tray icon provides quick actions for "New Note", "Save", and more, plus a dynamic list of **Recently Closed Files** for one-click reopening.
*   📚 **Batch File Opening:** Select and open multiple files at once from the file dialog. QuickNote will neatly open each file in its own new tab, ready for you to work on.

## 🛠️ Tech Stack

| Area      | Technology                                                                                                                                                                                                  |
| :-------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Core**  | ![Tauri](https://img.shields.io/badge/Tauri-24C8DB?logo=tauri&logoColor=white) ![Rust](https://img.shields.io/badge/Rust-DEA584?logo=rust&logoColor=white)                                                     |
| **Frontend** | ![Vue.js](https://img.shields.io/badge/Vue.js-4FC08D?logo=vue.js&logoColor=white) ![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white)                              |
| **State** | ![Pinia](https://img.shields.io/badge/Pinia-FFD700?logo=pinia&logoColor=black)                                                                                                                                |
| **Build** | ![Vite](https://img.shields.io/badge/Vite-646CFF?logo=vite&logoColor=white)                                                                                                                                    |

## 📦 Download

[⬇️ Download for macOS (.dmg)](https://github.com/kaigii/QuickNote/releases/tag/v0.1.0)

Or visit the [Releases page for all versions](https://github.com/kaigii/QuickNote/releases)

### Installation Notes

Since this is an unsigned application, macOS may block the installation. If you see "QuickNote is damaged and can't be opened" error:

1. **Method 1 (Recommended)**: Right-click the `.dmg` file and select "Open"
2. **Method 2**: Go to System Preferences → Security & Privacy → General, and click "Open Anyway"
3. **Method 3 (Terminal)**: Run this command in Terminal:
   ```bash
   sudo xattr -rd com.apple.quarantine /path/to/QuickNote_0.1.0_aarch64.dmg
   ```
   Then try opening the DMG file again.

**Note**: Replace `/path/to/` with the actual path where you downloaded the file (usually `~/Downloads/`).

---

## 📦 Getting Started

### Prerequisites

-   [Node.js](https://nodejs.org/en/)
-   [Rust](https://www.rust-lang.org/tools/install) and Cargo
-   Follow the Tauri [prerequisites guide](https://tauri.app/v1/guides/getting-started/prerequisites) for your specific OS.

### Installation & Running

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/kaigii/QuickNote.git
    cd QuickNote
    ```

2.  **Install frontend dependencies:**
    ```bash
    npm install
    ```

3.  **Run in development mode:**
    ```bash
    npm run tauri dev
    ```

4.  **Build the application:**
    ```bash
    npm run tauri build
    ```

## 📜 License

This project is licensed under the **Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License (CC BY-NC-SA 4.0)**.

This means:
-   You are free to **share** and **adapt** the material for non-commercial purposes.
-   You must give appropriate **credit** (Attribution).
-   You may **not** use the material for **commercial purposes** (NonCommercial).
-   If you remix, transform, or build upon the material, you must distribute your contributions under the **same license** (ShareAlike).

For the full license text, see [LICENSE](LICENSE) or visit the [Creative Commons website](https://creativecommons.org/licenses/by-nc-sa/4.0/).

## ❤️ Contributing

Contributions are welcome! Whether it's reporting a bug, suggesting a feature, or submitting a pull request, your help is appreciated.

1.  Fork the Project
2.  Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3.  Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4.  Push to the Branch (`git push origin feature/AmazingFeature`)
5.  Open a Pull Request