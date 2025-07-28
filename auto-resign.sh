#!/bin/bash

# QuickNote Auto-Resign Script
# Automatically rebuilds and re-signs the app every 7 days

echo "🔄 QuickNote Auto-Resign Script"
echo "================================"

# Check if we have signing identity
if ! security find-identity -v -p codesigning | grep -q "Apple Development"; then
    echo "❌ No Apple Development signing identity found"
    echo "Please run: ./sign-app.sh"
    exit 1
fi

# Check if we need to resign (check if DMG exists and is older than 6 days)
DMG_PATH="src-tauri/target/release/bundle/dmg/QuickNote_0.1.0_aarch64.dmg"

if [ -f "$DMG_PATH" ]; then
    # Get file modification time
    FILE_TIME=$(stat -f %m "$DMG_PATH")
    CURRENT_TIME=$(date +%s)
    AGE_DAYS=$(( (CURRENT_TIME - FILE_TIME) / 86400 ))
    
    echo "📅 Current DMG age: $AGE_DAYS days"
    
    if [ $AGE_DAYS -lt 6 ]; then
        echo "✅ DMG is still fresh (less than 6 days old)"
        echo "💡 No need to re-sign yet"
        echo "📅 Will need re-signing in $((6 - AGE_DAYS)) days"
        exit 0
    else
        echo "⚠️  DMG is $AGE_DAYS days old, re-signing..."
    fi
else
    echo "📦 No existing DMG found, building for the first time..."
fi

# Build and sign
echo "🔨 Building and signing QuickNote..."
if npm run tauri build; then
    echo "✅ Build and sign completed successfully!"
    echo "📁 New DMG location: $DMG_PATH"
    echo "📅 Valid until: $(date -v+7d '+%Y-%m-%d %H:%M:%S')"
    
    # Copy to desktop for easy access
    cp "$DMG_PATH" ~/Desktop/QuickNote_Signed_$(date +%Y%m%d).dmg
    echo "📋 Also copied to: ~/Desktop/QuickNote_Signed_$(date +%Y%m%d).dmg"
    
    echo ""
    echo "🎉 Ready for distribution!"
    echo "💡 Remember to upload to GitHub Releases"
else
    echo "❌ Build failed"
    exit 1
fi 