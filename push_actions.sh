#!/data/data/com.termux/files/usr/bin/bash
cd /data/data/com.termux/files/home/CamScanner || exit 1
echo "Pushing workflow file to trigger GitHub Actions APK build..."
git push origin main
