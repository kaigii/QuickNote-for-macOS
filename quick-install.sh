#!/bin/bash

# QuickNote 一鍵安裝腳本
# 自動處理所有安裝步驟

echo "🚀 QuickNote 一鍵安裝"
echo "======================"

# 尋找 DMG 檔案
DMG_FILE="QuickNote_0.1.0_aarch64.dmg"
FOUND_FILE=""

# 檢查常見位置
for path in "." "$HOME/Downloads" "$HOME/Desktop" "$HOME/Downloads/QuickNote*" "$HOME/Desktop/QuickNote*"; do
    if [ -f "$path/$DMG_FILE" ]; then
        FOUND_FILE="$path/$DMG_FILE"
        break
    fi
done

# 如果沒找到，搜尋整個下載資料夾
if [ -z "$FOUND_FILE" ]; then
    echo "🔍 搜尋 DMG 檔案..."
    FOUND_FILE=$(find "$HOME/Downloads" -name "*QuickNote*.dmg" 2>/dev/null | head -1)
fi

if [ -z "$FOUND_FILE" ]; then
    echo "❌ 找不到 QuickNote DMG 檔案"
    echo ""
    echo "請確保已下載 QuickNote_0.1.0_aarch64.dmg 到以下位置之一："
    echo "- 當前資料夾"
    echo "- 下載資料夾 (~/Downloads)"
    echo "- 桌面 (~/Desktop)"
    echo ""
    echo "或者手動指定檔案路徑："
    read -p "DMG 檔案路徑: " FOUND_FILE
    if [ ! -f "$FOUND_FILE" ]; then
        echo "❌ 檔案不存在"
        exit 1
    fi
fi

echo "📦 找到檔案: $FOUND_FILE"

# 自動移除隔離標記
echo "🔓 移除安全標記..."
if sudo xattr -rd com.apple.quarantine "$FOUND_FILE" 2>/dev/null; then
    echo "✅ 安全標記已移除"
else
    echo "⚠️  安全標記處理完成"
fi

# 自動掛載
echo "📂 掛載 DMG..."
if hdiutil attach "$FOUND_FILE" >/dev/null 2>&1; then
    echo "✅ DMG 已掛載"
    
    # 自動複製到 Applications
    echo "📋 複製到 Applications..."
    if cp -R "/Volumes/QuickNote/QuickNote.app" "/Applications/" 2>/dev/null; then
        echo "✅ 安裝完成！"
        echo ""
        echo "🎉 QuickNote 已安裝到 Applications 資料夾"
        echo "💡 現在可以從 Applications 啟動 QuickNote"
        
        # 詢問是否立即啟動
        read -p "是否立即啟動 QuickNote？(y/n): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo "🚀 啟動 QuickNote..."
            open "/Applications/QuickNote.app"
        fi
    else
        echo "⚠️  複製失敗，請手動安裝"
        echo "請將 QuickNote.app 從掛載的磁碟拖曳到 Applications 資料夾"
    fi
    
    # 卸載 DMG
    echo "📂 卸載 DMG..."
    hdiutil detach "/Volumes/QuickNote" >/dev/null 2>&1
else
    echo "❌ DMG 掛載失敗"
    exit 1
fi

echo ""
echo "✨ 安裝流程完成！" 