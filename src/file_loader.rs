use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;

use uefi::{CString16, Handle};
use uefi::proto::loaded_image;
use uefi::proto::media::{file, fs};
use uefi::proto::media::file::{File, FileAttribute, FileMode};
use uefi::table::{Boot, SystemTable};
use uefi_services::println;

pub struct FileLoader<'a> {
    image: &'a Handle,
    system_table: &'a SystemTable<Boot>,
}

impl<'a> FileLoader<'a> {
    pub fn new(image: &'a Handle, system_table: &'a SystemTable<Boot>) -> Self {
        FileLoader {
            image,
            system_table,
        }
    }

    pub fn read_file(&self, file_name: &str, directory: Option<&str>) -> Result<Vec<u8>, String> {
        let mut dir = prepare_file_system(self.image, self.system_table);
        if let Some(sub_dir) = directory {
            let dir_name = CString16::try_from(sub_dir).unwrap();
            match dir.open(&dir_name, FileMode::Read, FileAttribute::READ_ONLY) {
                Ok(fh) => dir = fh.into_directory().unwrap(),
                Err(e) => {
                    println!("{:?}", e);
                    return Err("Could not open directory!".to_owned());
                }
            }
        }
        let file_name = CString16::try_from(file_name).unwrap();
        if let Ok(handle) = dir.open(&file_name, FileMode::Read, FileAttribute::READ_ONLY) {
            if let Some(mut regular) = handle.into_regular_file() {
                let mut buf: [u8; 500_000] = [0; 500_000];
                match regular.read(&mut buf) {
                    Ok(bytes_read) => Ok(Vec::from(&buf[0..bytes_read])),
                    Err(_) => Err("Could not read file".to_owned())
                }
            } else {
                Err(format!("'{}' is not a regular file!", file_name).to_owned())
            }
        } else {
            Err("Can't open file".to_owned())
        }
    }
}


fn prepare_file_system(image: &Handle, system_table: &SystemTable<Boot>) -> file::Directory {
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