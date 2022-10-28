#!/usr/bin/env sh
rm -rf target/assets || true
mkdir target/assets
cp assets/* target/assets

hdiutil create -srcfolder target/assets -volname MASKOS -fs FAT32 -ov -format UDTO -layout NONE maskos