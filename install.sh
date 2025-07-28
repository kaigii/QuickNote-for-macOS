#!/bin/bash

# QuickNote for macOS 安裝腳本
# 此腳本會自動處理從 GitHub 下載的 DMG 檔案

echo "🚀 QuickNote for macOS 安裝腳本"
echo "=================================="

# 檢查 DMG 檔案是否存在
DMG_FILE="QuickNote_0.1.0_aarch64.dmg"
if [ ! -f "$DMG_FILE" ]; then
    echo "❌ 找不到 $DMG_FILE 檔案"
    echo "請確保 DMG 檔案與此腳本在同一資料夾"
    exit 1
fi

echo "📦 找到 DMG 檔案: $DMG_FILE"

# 移除隔離標記
echo "🔓 移除安全隔離標記..."
sudo xattr -rd com.apple.quarantine "$DMG_FILE"

if [ $? -eq 0 ]; then
    echo "✅ 安全標記已移除"
else
    echo "⚠️  移除安全標記時出現警告，但這通常是正常的"
fi

# 掛載 DMG
echo "📂 掛載 DMG 檔案..."
hdiutil attach "$DMG_FILE"

if [ $? -eq 0 ]; then
    echo "✅ DMG 已成功掛載"
    echo ""
    echo "🎉 安裝步驟："
    echo "1. 在 Finder 中找到掛載的 QuickNote 磁碟"
    echo "2. 將 QuickNote.app 拖曳到 Applications 資料夾"
    echo "3. 從 Applications 資料夾啟動 QuickNote"
    echo ""
    echo "💡 提示：如果啟動時出現安全警告，請前往"
    echo "   系統偏好設定 → 安全性與隱私 → 一般"
    echo "   點擊「仍要打開」"
else
    echo "❌ DMG 掛載失敗"
    exit 1
fi 