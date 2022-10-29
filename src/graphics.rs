use alloc::vec::Vec;

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
    fn pixel(&self, x: usize, y: usize) -> Color4;

    fn draw(&self, vfb: &mut VirtualFrameBuffer) {
        let p = self.pos();

        for x in 0..self.width() {
            for y in 0..self.height() {
                let (fb_x, fb_y) = (p[0] as usize + x, p[1] as usize + y);
                if fb_x < 0 || fb_x >= vfb.data.len() || fb_y < 0 || fb_y >= vfb.data[0].len() {
                    continue;
                }

                let color = self.pixel(x, y);
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
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            pixels: [[Color4::new(255, 255, 255, 255); 16]; 16],
        }
    }
}

pub struct Sprite {
    pub pos: Vec2,
    pub tiles: Vec<Vec<Tile>>,
}

impl Default for Sprite {
    fn default() -> Self {
        Sprite {
            pos: Vec2::new(0, 0),
            tiles: vec![vec![Tile::default()]],
        }
    }
}

impl DrawFramebuffer for Sprite {
    fn width(&self) -> usize {
        self.tiles.len() * Tile::WIDTH
    }

    fn height(&self) -> usize {
        self.tiles[0].len() * Tile::HEIGHT
    }

    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn pixel(&self, x: usize, y: usize) -> Color4 {
        let (tile_x, tile_y) = (x / Tile::WIDTH, y / Tile::HEIGHT);
        let (pixel_x, pixel_y) = (x % Tile::WIDTH, y % Tile::HEIGHT);

        self.tiles[tile_x][tile_y].pixels[pixel_x][pixel_y]
    }
}
