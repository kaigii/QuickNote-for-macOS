<div align="center">

# QuickNote

[English](#en) | [繁體中文](#zh)

![License](https://img.shields.io/badge/license-CC--BY--NC--SA--4.0-blue.svg)
![Version](https://img.shields.io/badge/version-0.1.0-brightgreen.svg)
![Tauri](https://img.shields.io/badge/Tauri-24C8DB?logo=tauri&logoColor=white)
![Vue.js](https://img.shields.io/badge/Vue.js-4FC08D?logo=vue.js&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-DEA584?logo=rust&logoColor=white)

**A simple, fast, and lightweight note-taking application designed for quick access and ease of use.**

</div>

---

<a id="en"></a>

<!-- English Section Start -->

### ✨ **QuickNote in Action**

*(Put a GIF demo of your app here, e.g., theme switching, new note, custom hotkeys, etc.)*

![QuickNote Demo](https://user-images.githubusercontent.com/.../placeholder.gif) 
> *Replace this with a GIF of your application.*

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

---

<a id="zh"></a>

## QuickNote (繁體中文版本)

[English](#en)

<div align="center">

**一款為快速存取與簡易使用而設計的、簡單且輕量的筆記應用程式。**

</div>

---

### ✨ **QuickNote 實際操作**

*(建議您在此處放置一個展示應用程式功能的 GIF 動畫，例如：主題切換、建立新筆記、自訂快捷鍵等)*

![QuickNote Demo](https://user-images.githubusercontent.com/.../placeholder.gif) 
> *請替換成您應用程式的 GIF 動畫。*

---

## 🚀 主要特色

QuickNote 不僅是另一個文字編輯器。它被精心打造，旨在成為您思維的延伸，幫助您無阻力地捕捉靈感，並提升您的生產力。

### 核心體驗與效能
*   ⚡️ **全域即時存取：** 無論您在作業系統的任何角落，都能透過可自訂的全域熱鍵，立即呼叫新筆記或切換工作區。在靈感湧現的瞬間捕捉它，無需離開您當前的工作。
*   💨 **原生級效能，羽量級佔用：** 基於 Rust 後端與 Tauri 核心打造，QuickNote 能即時啟動，並以極低的 CPU 和記憶體佔用率運行。體驗網頁應用無法比擬的原生速度與響應能力。
*   🧠 **智慧置頂，專注不中斷：** 當您使用快捷鍵呼叫 QuickNote 時，它會智慧地將視窗帶至最前並短暫置頂，確保您能立即開始輸入，而不會因焦點切換而丟失視窗。

### 直觀的現代化介面
*   🗂️ **進階分頁管理：** 透過強大的分頁介面組織您的思緒。輕鬆地**拖放**以重新排序分頁、**雙擊以重新命名**，並透過視覺標記 (`•`) 一目了然地查看未儲存的變更。
*   ✍️ **極簡而強大的編輯器：** 一個乾淨、無干擾的書寫環境，配備了如**行號**和**Tab 縮排**等基本工具，幫助您專注於最重要的事：您的內容。
*   🌓 **動態主題切換：** 在精美的**淺色**與舒適的**深色**主題之間無縫切換。您的選擇將被保存並立即應用，讓整個 UI 適應您的個人偏好。
*   🖼️ **時尚無邊框設計 (macOS)：** 在 macOS 上享受現代、統一的美學，具備自訂的無邊框視窗與功能完整的整合式**「紅綠燈」**控制按鈕，帶來真正的原生感受。
*   📂 **拖放開啟檔案：** 只需從桌面或檔案總管中拖曳檔案，並將其放置到 QuickNote 視窗上，即可立即在新分頁中開啟它們。

### 強大的自訂能力
*   ⌨️ **完全可配置的全域熱鍵：** 完全掌控您的工作流程，自訂用於建立新筆記、關閉分頁以及顯示/隱藏應用程式視窗的鍵盤快捷鍵。
*   ⚙️ **工作流程偏好設定：** 透過設定**預設儲存路徑**與**預設檔案格式**（如 `.txt`、`.md`）來簡化您的操作，最大程度地減少重複性動作。
*   🌐 **多國語言介面：** 即時切換**英文**與**繁體中文**。整個介面，從設定到狀態列，都將立即更新。

### 進階工作流程與生產力
*   💾 **智慧儲存系統：** QuickNote 能聰明地處理儲存。它對現有檔案使用`儲存`，對新的未命名筆記則自動提示`另存新檔...`，確保您不會意外覆蓋錯誤的檔案。
*   🛡️ **未儲存變更保護：** 再也不會丟失您的工作。在關閉有未儲存變更的分頁前，QuickNote 會發出警告並請求您的確認。
*   📦 **智慧型系統匣選單：** 無需開啟視窗即可存取核心功能。系統匣圖示提供「新增筆記」、「儲存」等快速操作，外加一個動態的**「最近關閉的檔案」**列表，讓您一鍵重新開啟。
*   📚 **批次檔案開啟：** 從檔案對話框中一次選取並開啟多個檔案。QuickNote 會將每個檔案整齊地在各自的新分頁中開啟，隨時供您使用。

## 🛠️ 技術棧

| 領域       | 技術                                                                                                                                                                                                     |
| :--------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **核心框架** | ![Tauri](https://img.shields.io/badge/Tauri-24C8DB?logo=tauri&logoColor=white) ![Rust](https://img.shields.io/badge/Rust-DEA584?logo=rust&logoColor=white)                                                      |
| **前端**     | ![Vue.js](https://img.shields.io/badge/Vue.js-4FC08D?logo=vue.js&logoColor=white) ![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white)                               |
| **狀態管理** | ![Pinia](https://img.shields.io/badge/Pinia-FFD700?logo=pinia&logoColor=black)                                                                                                                                 |
| **建置工具** | ![Vite](https://img.shields.io/badge/Vite-646CFF?logo=vite&logoColor=white)                                                                                                                                     |

## 📦 開始使用

### 環境需求

-   [Node.js](https://nodejs.org/en/)
-   [Rust](https://www.rust-lang.org/tools/install) 與 Cargo
-   根據您的作業系統，遵循 Tauri 的[環境準備指南](https://tauri.app/v1/guides/getting-started/prerequisites)。

### 安裝與執行

1.  **複製專案倉庫：**
    ```bash
    git clone https://github.com/kaigii/QuickNote.git
    cd QuickNote
    ```

2.  **安裝前端依賴：**
    ```bash
    npm install
    ```

3.  **以開發模式執行：**
    ```bash
    npm run tauri dev
    ```

4.  **建置應用程式：**
    ```bash
    npm run tauri build
    ```

## 📜 授權條款

本專案採用 **Creative Commons 姓名標示-非商業性-相同方式分享 4.0 國際 (CC BY-NC-SA 4.0)** 授權。

這意味著：
-   您可以自由地**分享**與**改作**本專案，但僅限於非商業性用途。
-   您必須給予適當的**姓名標示** (Attribution)。
-   您**不得**將本作品用於**商業目的** (NonCommercial)。
-   如果您混合、轉換或基於本作品創作，您必須基於**相同授權條款**來散布您的貢獻 (ShareAlike)。

完整的授權條款請參閱 [LICENSE](LICENSE) 檔案或訪問 [Creative Commons 網站](https://creativecommons.org/licenses/by-nc-sa/4.0/deed.zh_TW)。

## ❤️ 貢獻

歡迎任何形式的貢獻！無論是回報錯誤、建議新功能，或是提交 Pull Request，我們都非常感謝您的幫助。

1.  Fork 本專案
2.  建立您的功能分支 (`git checkout -b feature/AmazingFeature`)
3.  提交您的變更 (`git commit -m 'Add some AmazingFeature'`)
4.  將分支推送到遠端 (`git push origin feature/AmazingFeature`)
5.  建立一個 Pull Request