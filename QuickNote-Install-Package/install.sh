#!/bin/bash

# QuickNote One-Click Installation Script
# Automatically handles all installation steps

echo "🚀 QuickNote One-Click Installation"
echo "===================================="

# Find DMG file
DMG_FILE="QuickNote_0.1.0_aarch64.dmg"
FOUND_FILE=""

# Check common locations
for path in "." "$HOME/Downloads" "$HOME/Desktop" "$HOME/Downloads/QuickNote*" "$HOME/Desktop/QuickNote*"; do
    if [ -f "$path/$DMG_FILE" ]; then
        FOUND_FILE="$path/$DMG_FILE"
        break
    fi
done

# If not found, search the entire Downloads folder
if [ -z "$FOUND_FILE" ]; then
    echo "🔍 Searching for DMG file..."
    FOUND_FILE=$(find "$HOME/Downloads" -name "*QuickNote*.dmg" 2>/dev/null | head -1)
fi

# If still not found, offer to download automatically
if [ -z "$FOUND_FILE" ]; then
    echo "📦 QuickNote DMG file not found locally"
    echo ""
    echo "Would you like to download it automatically from GitHub? (y/n): "
    read -p "" -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "📥 Downloading QuickNote from GitHub..."
        DOWNLOAD_URL="https://github.com/kaigii/QuickNote-for-macOS/releases/download/v0.1.0/QuickNote_0.1.0_aarch64.dmg"
        
        # Download to Downloads folder
        if curl -L -o "$HOME/Downloads/$DMG_FILE" "$DOWNLOAD_URL"; then
            echo "✅ Download completed!"
            FOUND_FILE="$HOME/Downloads/$DMG_FILE"
        else
            echo "❌ Download failed"
            echo ""
            echo "Please download manually from:"
            echo "https://github.com/kaigii/QuickNote-for-macOS/releases"
            exit 1
        fi
    else
        echo ""
        echo "Please ensure QuickNote_0.1.0_aarch64.dmg is downloaded to one of these locations:"
        echo "- Current folder"
        echo "- Downloads folder (~/Downloads)"
        echo "- Desktop (~/Desktop)"
        echo ""
        echo "Or manually specify the file path:"
        read -p "DMG file path: " FOUND_FILE
        if [ ! -f "$FOUND_FILE" ]; then
            echo "❌ File does not exist"
            exit 1
        fi
    fi
fi

echo "📦 Found file: $FOUND_FILE"

# Remove quarantine attributes
echo "🔓 Removing security quarantine..."
if sudo xattr -rd com.apple.quarantine "$FOUND_FILE" 2>/dev/null; then
    echo "✅ Security quarantine removed"
else
    echo "⚠️  Security quarantine processing completed"
fi

# Mount DMG
echo "📂 Mounting DMG..."
if hdiutil attach "$FOUND_FILE" >/dev/null 2>&1; then
    echo "✅ DMG mounted"
    
    # Copy to Applications
    echo "📋 Copying to Applications..."
    if cp -R "/Volumes/QuickNote/QuickNote.app" "/Applications/" 2>/dev/null; then
        echo "✅ Installation completed!"
        echo ""
        echo "🎉 QuickNote has been installed to Applications folder"
        echo "💡 You can now launch QuickNote from Applications"
        
        # Ask if user wants to launch immediately
        read -p "Launch QuickNote now? (y/n): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo "🚀 Launching QuickNote..."
            open "/Applications/QuickNote.app"
        fi
    else
        echo "⚠️  Copy failed, please install manually"
        echo "Please drag QuickNote.app from the mounted disk to Applications folder"
    fi
    
    # Unmount DMG
    echo "📂 Unmounting DMG..."
    hdiutil detach "/Volumes/QuickNote" >/dev/null 2>&1
else
    echo "❌ DMG mount failed"
    exit 1
fi

echo ""
echo "✨ Installation process completed!" 