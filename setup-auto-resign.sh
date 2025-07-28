#!/bin/bash

# Setup Auto-Resign Cron Job
# Sets up automatic re-signing every 6 days

echo "⏰ QuickNote Auto-Resign Setup"
echo "=============================="

# Get the current directory
CURRENT_DIR=$(pwd)
AUTO_RESIGN_SCRIPT="$CURRENT_DIR/auto-resign.sh"

echo "📁 Project directory: $CURRENT_DIR"
echo "📜 Auto-resign script: $AUTO_RESIGN_SCRIPT"

# Check if auto-resign script exists
if [ ! -f "$AUTO_RESIGN_SCRIPT" ]; then
    echo "❌ Auto-resign script not found"
    exit 1
fi

# Make sure it's executable
chmod +x "$AUTO_RESIGN_SCRIPT"

echo ""
echo "📋 Available options:"
echo "1. Set up daily check (recommended)"
echo "2. Set up weekly check"
echo "3. Manual check only"
echo "4. Remove existing cron job"

read -p "Choose option (1-4): " choice

case $choice in
    1)
        echo "⏰ Setting up daily check..."
        # Add to crontab - run daily at 9 AM
        (crontab -l 2>/dev/null; echo "0 9 * * * cd $CURRENT_DIR && $AUTO_RESIGN_SCRIPT >> auto-resign.log 2>&1") | crontab -
        echo "✅ Daily cron job added (runs at 9 AM daily)"
        ;;
    2)
        echo "⏰ Setting up weekly check..."
        # Add to crontab - run every Sunday at 9 AM
        (crontab -l 2>/dev/null; echo "0 9 * * 0 cd $CURRENT_DIR && $AUTO_RESIGN_SCRIPT >> auto-resign.log 2>&1") | crontab -
        echo "✅ Weekly cron job added (runs every Sunday at 9 AM)"
        ;;
    3)
        echo "📝 Manual check instructions:"
        echo "Run this command when you want to check/re-sign:"
        echo "  ./auto-resign.sh"
        echo ""
        echo "Or check if re-signing is needed:"
        echo "  ./auto-resign.sh"
        ;;
    4)
        echo "🗑️  Removing existing cron jobs..."
        crontab -l 2>/dev/null | grep -v "auto-resign.sh" | crontab -
        echo "✅ Cron jobs removed"
        ;;
    *)
        echo "❌ Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "📋 Current cron jobs:"
crontab -l 2>/dev/null | grep -E "(auto-resign|QuickNote)" || echo "No QuickNote cron jobs found"

echo ""
echo "💡 Tips:"
echo "- Check logs: tail -f auto-resign.log"
echo "- Manual run: ./auto-resign.sh"
echo "- View cron jobs: crontab -l" 