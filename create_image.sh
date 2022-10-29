#!/usr/bin/env sh
rm -rf target/assets || true
mkdir target/assets
cp -r assets/* target/assets/
cp target/x86_64-unknown-uefi/release/uefi_app.efi target/assets/

hdiutil create -srcfolder target/assets -volname MASKOS -fs FAT32 -ov -format UDTO -layout NONE maskos
