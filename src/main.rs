#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(lang_items)]

#[macro_use]
extern crate alloc;

pub mod math;
pub mod graphics;
pub mod file_loader;


use uefi::prelude::*;
use uefi_services::println;
use uefi::table::boot::{OpenProtocolAttributes, OpenProtocolParams, EventType, Tpl, TimerTrigger};
use uefi::proto::console::gop::{BltOp, BltPixel, FrameBuffer, GraphicsOutput, PixelFormat};
use uefi::proto::console::text::{Key, ScanCode};

use math::{Vec2, Color4};
use graphics::{VirtualFrameBuffer, DrawFramebuffer, Tile, TileSet, Sprite, Level};
use crate::file_loader::{FileLoader};

#[entry]
unsafe fn main(image: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut st).expect("failed to init uefi services");

    let st_clone = st.unsafe_clone();
    let bt = st_clone.boot_services();

    if let Ok(handle) = bt.get_handle_for_protocol::<GraphicsOutput>() {
        let gop = &mut bt
                .open_protocol::<GraphicsOutput>(
                    OpenProtocolParams {
                        handle,
                        agent: image,
                        controller: None,
                    },
                    // For this character, don't open in exclusive mode. That
                    // would break the connection between stdout and the
                    // video console.
                    OpenProtocolAttributes::GetProtocol,
                )
                .expect("failed to open Graphics Output Protocol");

        println!("GOP inited succesfully!");

        let (width, height) = choose_graphics_mode(gop, st.unsafe_clone(), bt);

        let mi = gop.current_mode_info();
        let stride = mi.stride();

        let mut fb = gop.frame_buffer();
        
        /* game loop */
        let mut vfb = VirtualFrameBuffer::new();
        let mut file_loader = FileLoader::new(&image, &st);
        let tile_set_bytes = file_loader.read_file( "TileSet.bmp", None).unwrap();
        let mut tile_set = TileSet::new_from_buffer(tile_set_bytes);

        let level1 = Level::new_from_name(&file_loader, "1");

        println!("Beginning game loop");

        let mut move_dir = Vec2::new(0, 0);
        let mut character = Sprite::default();

        let mut test = Sprite::default();
        test.pos = Vec2::new(100, 100);

        loop {
            bt.stall(1000);

            match st.stdin().read_key().unwrap() {
                Some(Key::Special(ScanCode::LEFT)) => {
                    move_dir = if move_dir[0] == 1 {
                        Vec2::new(0, 0)
                    } else {
                        Vec2::new(-1, 0)
                    };
                },
                Some(Key::Special(ScanCode::RIGHT)) => {
                    move_dir = if move_dir[0] == -1 {
                        Vec2::new(0, 0)
                    } else {
                        Vec2::new(1, 0)
                    };
                },
                Some(Key::Special(ScanCode::UP)) => {
                    move_dir = if move_dir[1] == 1 {
                        Vec2::new(0, 0)
                    } else {
                        Vec2::new(0, -1)
                    };
                },
                Some(Key::Special(ScanCode::DOWN)) => {
                    move_dir = if move_dir[1] == -1 {
                        Vec2::new(0, 0)
                    } else {
                        Vec2::new(0, 1)
                    };
                },
                _ => ()
            }

            bt.stall(1000);

            if let Some(entities) = level1.collides(&character, move_dir) {
                // Handle collision: check if all walls have the correct color(s) and if so,
                // move the character here, too.
            } else {
                character.pos += move_dir;
            }

            vfb.clear(Color4::new(0, 0, 0, 255));

            level1.sprite.draw(&tile_set, &mut vfb);

            character.draw(&tile_set, &mut vfb);
            test.draw(&tile_set, &mut vfb);

            draw_vfb_to_fb(&mut fb, stride, &vfb);

            bt.stall(1000);
        }
    } else {
        println!("GOP not supported!");
        panic!();
    }

    //Status::SUCCESS
}

fn choose_graphics_mode(gop: &mut GraphicsOutput, mut st: SystemTable<Boot>, bt: &BootServices) -> (usize, usize){
    let mut mode_index = usize::MAX;
    for i in 0..gop.modes().len() {
        let res = gop.modes().nth(i).unwrap().info().resolution();
        if let (640, 480) = res {
            mode_index = i;
        }
    }

    if mode_index == usize::MAX {
        panic!("resolution 640x480 is not available");
    }

    //let timer = bt.create_event(EventType::TIMER, Tpl::APPLICATION, None, None);
    //println!("Timer status: {:?}", timer.status());
    //let timer = timer.unwrap();
    //bt.set_timer(&timer, TimerTrigger::Relative(50000000)).expect("Could'nt set timer");

    //bt.wait_for_event(&mut [timer]).expect("Failed while wating for timer");
    //bt.wait_for_event(&mut [(*system_table.unsafe_clone().stdin().wait_for_key_event()).unsafe_clone()]).expect("Failed while waiting for key");

    //if let Some(Key::Printable(character)) = st.stdin().read_key().unwrap() {
    //    let character : char = character.into();
    //    if let Some(index) = character.to_digit(16) {
    //        mode_index = index as usize;
    //    } else {
    //        println!("Invalid character, choosing default mode");
    //    }
    //} else {
    //    println!("No or invalid character, choosing default mode");
    //}

    let mode = gop.modes().nth(mode_index).unwrap();

    let (width, height) = mode.info().resolution();

    gop.set_mode(&mode).expect("failed to set graphics mode");

    (width, height)
}

fn draw_vfb_to_fb(fb: &mut FrameBuffer, stride: usize, vfb: &VirtualFrameBuffer) {
    for x in 0..vfb.data.len() {
        for y in 0..vfb.data[0].len() {
            let pixel_index = (y * stride) + x;
            let pixel_base = 4 * pixel_index;
            let color = vfb.data[x][y];

            unsafe {
                fb.write_value(pixel_base, [color[0] as u8, color[1] as u8, color[2] as u8]);
            }
        }
    }
}

