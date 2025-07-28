#!/bin/bash

# QuickNote Code Signing Script
# This script helps sign the QuickNote application

echo "🔐 QuickNote Code Signing Helper"
echo "================================="

# Check if Xcode command line tools are installed
if ! command -v codesign &> /dev/null; then
    echo "❌ Xcode command line tools not found"
    echo "Please install Xcode command line tools first:"
    echo "xcode-select --install"
    exit 1
fi

# Check if we have a valid signing identity
echo "🔍 Checking available signing identities..."
security find-identity -v -p codesigning

echo ""
echo "📋 Available options:"
echo "1. Apple Development (free, 7-day validity)"
echo "2. Apple Distribution (requires paid developer account)"
echo "3. Skip signing (current behavior)"

read -p "Choose option (1-3): " choice

case $choice in
    1)
        echo "🔐 Using Apple Development signing..."
        # Update tauri.conf.json for development signing
        sed -i '' 's/"signingIdentity": "Apple Development"/"signingIdentity": "Apple Development"/' src-tauri/tauri.conf.json
        sed -i '' 's/"notarize": false/"notarize": false/' src-tauri/tauri.conf.json
        echo "✅ Configuration updated for development signing"
        echo "⚠️  Note: Development signed apps expire in 7 days"
        ;;
    2)
        echo "🔐 Using Apple Distribution signing..."
        # Update tauri.conf.json for distribution signing
        sed -i '' 's/"signingIdentity": "Apple Development"/"signingIdentity": "Apple Distribution"/' src-tauri/tauri.conf.json
        sed -i '' 's/"notarize": false/"notarize": true/' src-tauri/tauri.conf.json
        echo "✅ Configuration updated for distribution signing"
        echo "⚠️  Note: You need a paid Apple Developer account for this"
        ;;
    3)
        echo "⏭️  Skipping signing..."
        # Revert to no signing
        sed -i '' 's/"signingIdentity": "Apple Development"/"signingIdentity": null/' src-tauri/tauri.conf.json
        sed -i '' 's/"hardenedRuntime": true/"hardenedRuntime": false/' src-tauri/tauri.conf.json
        echo "✅ Configuration reverted to no signing"
        ;;
    *)
        echo "❌ Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "🚀 Next steps:"
echo "1. Run: npm run tauri build"
echo "2. The DMG will be created in src-tauri/target/release/bundle/dmg/"
echo ""
echo "💡 For permanent solution:"
echo "- Get Apple Developer account ($99/year)"
echo "- Use Apple Distribution signing"
echo "- Enable notarization" 