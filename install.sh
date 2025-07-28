#!/bin/bash

# QuickNote One-Click Installation Script
# Automatically handles all installation steps

echo "🚀 QuickNote One-Click Installation"
echo "===================================="

# Check if DMG file exists in current directory
DMG_FILE="QuickNote_0.1.0_aarch64.dmg"

if [ ! -f "$DMG_FILE" ]; then
    echo "❌ QuickNote DMG file not found in current directory"
    echo ""
    echo "📥 Please follow these steps:"
    echo "1. Go to GitHub Releases: https://github.com/kaigii/QuickNote-for-macOS/releases"
    echo "2. Download QuickNote_0.1.0_aarch64.dmg"
    echo "3. Save it to this directory (where install.sh is located)"
    echo "4. Run this script again: ./install.sh"
    echo ""
    echo "💡 Or if you already downloaded it elsewhere, move it here:"
    echo "   mv ~/Downloads/QuickNote_0.1.0_aarch64.dmg ."
    echo ""
    exit 1
fi

echo "📦 Found DMG file: $DMG_FILE"

# Remove quarantine attributes
echo "🔓 Removing security quarantine..."
if sudo xattr -rd com.apple.quarantine "$DMG_FILE" 2>/dev/null; then
    echo "✅ Security quarantine removed"
else
    echo "⚠️  Security quarantine processing completed"
fi

# Mount DMG
echo "📂 Mounting DMG..."
if hdiutil attach "$DMG_FILE" >/dev/null 2>&1; then
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