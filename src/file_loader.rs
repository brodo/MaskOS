use alloc::string::String;
use alloc::vec::Vec;
use uefi::{Char16, CStr16, CString16, Handle};
use uefi::proto::loaded_image;
use uefi::proto::media::{file, fs};
use uefi::proto::media::file::{File, FileAttribute, FileMode};
use uefi::table::{Boot, SystemTable};
use uefi_services::println;


pub fn read_file(image: &Handle, system_table: &SystemTable<Boot>) {
    let mut dir = prepare_file_system(image, system_table);

    let file_name = CString16::try_from("test.txt").unwrap();
    if let Ok(handle) = dir.open(&file_name, FileMode::Read, FileAttribute::READ_ONLY) {
        if let Some(mut regular) = handle.into_regular_file() {
            let mut buf: [u8; 500] = [0; 500];
            let bytes_read = regular.read(&mut buf).unwrap();
            let utf_8_str = core::str::from_utf8(&buf[..bytes_read]).unwrap();

            match CString16::try_from(utf_8_str) {
                Ok(cstr) => println!("Read string: {}", cstr),
                Err(e) => println!("{:?}", e)
            }
        }
    }
}


pub fn prepare_file_system(image: &Handle, system_table: &SystemTable<Boot>) -> file::Directory {
    let loaded_image = system_table
        .boot_services()
        .open_protocol_exclusive::<loaded_image::LoadedImage>(*image)
        .expect("Failed to load image");

    let mut simple_file_system = system_table
        .boot_services()
        .open_protocol_exclusive::<fs::SimpleFileSystem>(loaded_image.device()) // error: field `device_handle` of struct `uefi::proto::loaded_image::LoadedImage` is private
        .expect("Failed to prepare simple file system.");


    simple_file_system
        .open_volume()
        .expect("Failed to open volume.")
}