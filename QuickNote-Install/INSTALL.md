# 🚀 QuickNote for macOS 安裝指南

## 📦 下載
從 [GitHub Releases](https://github.com/kaigii/QuickNote/releases/tag/v0.1.0) 下載 `QuickNote_0.1.0_aarch64.dmg`

## ⚠️ 重要：解決「已損毀」錯誤

從 GitHub 下載的 DMG 檔案可能會顯示「已損毀，無法打開」錯誤。這是 macOS 的安全機制，檔案本身沒有問題。

### 🎯 快速解決方法

1. **開啟終端機** (按 `Cmd + 空白鍵` 搜尋 "Terminal")

2. **前往下載資料夾**：
   ```bash
   cd ~/Downloads
   ```

3. **修復 DMG 檔案**：
   ```bash
   sudo xattr -rd com.apple.quarantine QuickNote_0.1.0_aarch64.dmg
   ```

4. **輸入密碼**（輸入時不會顯示字元）

5. **現在可以正常打開 DMG 檔案**

### 🔧 其他方法

#### 方法 1：右鍵選單
- 右鍵點擊 DMG 檔案
- 選擇「打開」
- 如果出現警告，點擊「打開」

#### 方法 2：系統設定
- 前往「系統偏好設定」→「安全性與隱私」→「一般」
- 點擊「仍要打開」

#### 方法 3：使用安裝腳本
如果專案中有 `install.sh` 檔案：
```bash
chmod +x install.sh
./install.sh
```

## 📋 安裝步驟

1. **打開 DMG 檔案**
2. **將 QuickNote.app 拖曳到 Applications 資料夾**
3. **從 Applications 資料夾啟動 QuickNote**

## 🚨 如果應用程式無法啟動

如果從 Applications 資料夾啟動時出現安全警告：

1. 前往「系統偏好設定」→「安全性與隱私」→「一般」
2. 點擊「仍要打開」
3. 或者右鍵點擊應用程式 → 選擇「打開」

## 💡 為什麼會這樣？

- 這是 macOS 的正常安全機制
- 未簽名的應用程式會受到額外檢查
- 檔案本身沒有損壞，只是被標記為「隔離」
- 移除隔離標記後即可正常使用

## 🆘 需要幫助？

如果遇到問題，請：
1. 確認 macOS 版本（建議 10.15 或更新版本）
2. 檢查是否有足夠的磁碟空間
3. 在 GitHub Issues 中回報問題 