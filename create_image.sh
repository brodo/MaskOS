#!/usr/bin/env sh
cp startup.nsh target/x86_64-unknown-uefi/release/
hdiutil create -srcfolder target/x86_64-unknown-uefi/release/ -volname MASKOS -fs FAT32 -ov -format UDTO -layout NONE maskos