#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(lang_items)]

#[macro_use]
extern crate alloc;

pub mod math;
pub mod graphics;

use uefi::prelude::*;
use uefi_services::println;
use uefi::table::boot::{OpenProtocolAttributes, OpenProtocolParams, EventType, Tpl, TimerTrigger};
use uefi::proto::console::gop::{BltOp, BltPixel, FrameBuffer, GraphicsOutput, PixelFormat};
use uefi::proto::console::text::Key;

#[entry]
unsafe fn main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).expect("failed to init uefi services");

    let bt = system_table.boot_services();

    if let Ok(handle) = bt.get_handle_for_protocol::<GraphicsOutput>() {
        let gop = &mut bt
                .open_protocol::<GraphicsOutput>(
                    OpenProtocolParams {
                        handle,
                        agent: image,
                        controller: None,
                    },
                    // For this test, don't open in exclusive mode. That
                    // would break the connection between stdout and the
                    // video console.
                    OpenProtocolAttributes::GetProtocol,
                )
                .expect("failed to open Graphics Output Protocol");

        println!("GOP inited succesfully!");

        let (width, height) = choose_graphics_mode(gop, system_table.unsafe_clone(), bt);

        for i in 0..60 {
            fill_color(gop, i*123, i*252, i*184, width, height);
            bt.stall(16666);
        }
        draw_fb(gop, width, height);
    }
    else{
        println!("GOP not supported!");
        panic!();
    }

    Status::SUCCESS
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

fn fill_color(gop: &mut GraphicsOutput, r: u8, g: u8, b: u8, width: usize, height: usize) {
    let op = BltOp::VideoFill {
        color: BltPixel::new(r, g, b),
        dest: (0, 0),
        dims: (width, height),
    };

    gop.blt(op).expect("Failed to fill screen with color");
}

fn draw_fb(gop: &mut GraphicsOutput, width: usize, height: usize) {
    let mi = gop.current_mode_info();
    let stride = mi.stride();

    let mut fb = gop.frame_buffer();

    type PixelWriter = unsafe fn(&mut FrameBuffer, usize, [u8; 3]);
    unsafe fn write_pixel_rgb(fb: &mut FrameBuffer, pixel_base: usize, rgb: [u8; 3]) {
        fb.write_value(pixel_base, rgb);
    }
    unsafe fn write_pixel_bgr(fb: &mut FrameBuffer, pixel_base: usize, rgb: [u8; 3]) {
        fb.write_value(pixel_base, [rgb[2], rgb[1], rgb[0]]);
    }
    let write_pixel: PixelWriter = match mi.pixel_format() {
        PixelFormat::Rgb => write_pixel_rgb,
        PixelFormat::Bgr => write_pixel_bgr,
        _ => {
            println!("This pixel format is not supported by the drawing demo");
            return;
        }
    };

    let mut fill_rectangle = |(x1, y1), (x2, y2), color| {
        assert!((x1 < width) && (x2 < width), "Bad X coordinate");
        assert!((y1 < height) && (y2 < height), "Bad Y coordinate");
        for row in y1..y2 {
            for column in x1..x2 {
                unsafe {
                    let pixel_index = (row * stride) + column;
                    let pixel_base = 4 * pixel_index;
                    write_pixel(&mut fb, pixel_base, color);
                }
            }
        }
    };

    let mut x1: usize = 8;
    let mut y1: usize = 8;

    let mut r: u8 = 231;
    let mut g: u8 = 242;
    let mut b: u8 = 137;

    while x1*2 < width && y1*2 < height {
        x1 *= 2;
        y1 *= 2;

        r += 231;
        g += 242;
        b += 137;

        fill_rectangle((x1, y1), (width - x1, height - y1), [r, g, b]);
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
