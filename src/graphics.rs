use alloc::vec::Vec;
use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics::Pixel;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::prelude::Point;
use tinybmp::{Bmp, Pixels};

use uefi::CStr16;
use uefi_services::println;

use crate::math::{Vec2, Color4};

pub struct VirtualFrameBuffer {
    pub data: Vec<Vec<Color4>>,
}

impl VirtualFrameBuffer {
    pub fn new() -> VirtualFrameBuffer {
        VirtualFrameBuffer {
            data: vec![vec![Color4::new(0, 0, 0, 255); 480]; 640],
        }
    }

    pub fn clear(&mut self, clear_color: Color4) {
        for x in 0..self.data.len() {
            for y in 0..self.data[0].len() {
                self.data[x][y] = clear_color;
            }
        }
    }
}

pub trait DrawFramebuffer {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pos(&self) -> Vec2;
    fn pixel(&self, tile_set: &TileSet, x: usize, y: usize) -> Color4;

    fn draw(&self, tile_set: &TileSet, vfb: &mut VirtualFrameBuffer) {
        let p = self.pos();

        for x in 0..self.width() {
            for y in 0..self.height() {
                let (fb_x, fb_y) = (p[0] as usize + x, p[1] as usize + y);
                if fb_x < 0 || fb_x >= vfb.data.len() || fb_y < 0 || fb_y >= vfb.data[0].len() {
                    continue;
                }

                let color = self.pixel(tile_set, x, y);
                if color[3] != 0 {
                    vfb.data[fb_x][fb_y] = color;
                }
            }
        }
    }
}

/*
 * Convention: (r, g, b, a)
 *
 * The a (alpha) channel is fully transparent (not rendered) if 0, otherwise it is
 * fully opaque.
 */
pub struct Tile {
    pixels: [[Color4; 16]; 16],
}

impl Tile {
    pub const WIDTH: usize = 16;
    pub const HEIGHT: usize = 16;
    fn new_from_pixels(pixels: [[Color4; 16]; 16]) -> Self{
        Tile {
            pixels
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            pixels: [[Color4::new(255, 255, 255, 255); 16]; 16],
        }
    }
}

pub struct TileSet {
    pub tiles: Vec<Tile>,
}

impl TileSet {
    pub fn new_from_buffer(buffer: Vec<u8>) -> Self {
        let bmp = Bmp::<Rgb888>::from_slice(buffer.as_slice()).unwrap();
        let mut tiles: Vec<Tile> = vec![];
        for tile_x in 0..(bmp.size().width / 16) {
            for tile_y in 0..(bmp.size().height / 16) {
                let mut tile_bitmap = [[Color4::new(255, 255, 255, 255); 16]; 16];
                for x in 0..16 {
                    let mut row = tile_bitmap[x as usize];
                    for y in 0..16 {
                        let point = Point::new((tile_x + x) as i32, (tile_y + y) as i32);
                        let pixel = bmp.pixel(point).unwrap();
                        row[y as usize] = Color4::new(pixel.r().into(), pixel.g().into(), pixel.b().into(), 255);
                    }
                }
                tiles.push(Tile::new_from_pixels(tile_bitmap));
            }
        }

        TileSet {
            tiles,
        }
    }
}

impl Default for TileSet {
    fn default() -> Self {
        TileSet {
            tiles: vec![Tile::default()],
        }
    }
}

pub struct Entity {
    pub tile_index: usize,
    pub wall: bool,
    pub door_colors: Vec<usize>,
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            tile_index: 0,
            wall: false,
            door_colors: vec![],
        }
    }
}

pub struct Sprite {
    pub pos: Vec2,
    pub entities: Vec<Vec<Entity>>,
}

impl Default for Sprite {
    fn default() -> Self {
        Sprite {
            pos: Vec2::new(0, 0),
            entities: vec![vec![Entity::default()]],
        }
    }
}

impl DrawFramebuffer for Sprite {
    fn width(&self) -> usize {
        self.entities.len() * Tile::WIDTH
    }

    fn height(&self) -> usize {
        self.entities[0].len() * Tile::HEIGHT
    }

    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn pixel(&self, tile_set: &TileSet, x: usize, y: usize) -> Color4 {
        let (tile_x, tile_y) = (x / Tile::WIDTH, y / Tile::HEIGHT);
        let (pixel_x, pixel_y) = (x % Tile::WIDTH, y % Tile::HEIGHT);

        tile_set.tiles[self.entities[tile_x][tile_y].tile_index].pixels[pixel_x][pixel_y]
    }
}
