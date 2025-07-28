<div align="center">

# QuickNote for macOS（繁體中文版）

[English](./README.md)

**一款專為 macOS 設計，快速存取與簡易使用的輕量級筆記應用程式。**

</div>

---

### ✨ **QuickNote 實際操作**

**淺色主題**
![QuickNote Dark Theme](https://github.com/user-attachments/assets/8eda5e60-601d-467c-8861-0eb536d9e58c)

**深色主題**
![QuickNote Light Theme](https://github.com/user-attachments/assets/355d944b-36b7-40d5-9959-40ca30dc9370)

**設定面板（淺色）**
![QuickNote Settings Dark](https://github.com/user-attachments/assets/27a36411-048e-4f71-b339-1c85b1eda738)

**設定面板（深色）**
![QuickNote Settings Light](https://github.com/user-attachments/assets/6c5bb556-3701-46e8-8cc3-5a476a1accf1)

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

## 📦 下載

[⬇️ 下載 macOS 版本 (.dmg)](https://github.com/kaigii/QuickNote/releases/tag/v0.1.0)

或前往 [所有版本下載頁](https://github.com/kaigii/QuickNote/releases)

### 安裝說明

由於這是未簽名的應用程式，macOS 可能會阻擋安裝。如果出現「QuickNote」已損毀，無法打開」錯誤：

#### 🚀 快速安裝（推薦）

**選項 1：一鍵安裝（最簡單）**
```bash
chmod +x install.sh
./install.sh
```

**選項 2：手動指令**
1. 從 GitHub 下載 DMG 檔案
2. 開啟終端機並執行以下指令（請替換路徑）：
   ```bash
   sudo xattr -rd com.apple.quarantine /path/to/QuickNote_0.1.0_aarch64.dmg
   ```
   
   **常見路徑：**
   - 下載資料夾：`~/Downloads/QuickNote_0.1.0_aarch64.dmg`
   - 桌面：`~/Desktop/QuickNote_0.1.0_aarch64.dmg`
   - 當前資料夾：`./QuickNote_0.1.0_aarch64.dmg`

3. 現在可以正常打開 DMG 檔案

#### 🔧 其他方法
1. **方法 1**：右鍵點擊 `.dmg` 檔案並選擇「打開」
2. **方法 2**：前往系統偏好設定 → 安全性與隱私 → 一般，點擊「仍要打開」
3. **方法 3**：使用提供的安裝腳本：
   ```bash
   chmod +x install.sh
   ./install.sh
   ```

#### 💡 為什麼會這樣
這是從網路下載未簽名應用程式的正常行為。macOS 會添加安全隔離屬性來保護用戶。檔案實際上並沒有損壞。

---

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