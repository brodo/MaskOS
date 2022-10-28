use crate::math::{Vec2, Vec4};
use alloc::vec::Vec;

/*
 * Convention: (r, g, b, a)
 *
 * The a (alpha) channel is fully transparent (not rendered) if 0, otherwise it is
 * fully opaque.
 */
pub struct Tile {
    pixels: [[Vec4; 16]; 16],
}

impl Tile {
    pub const WIDTH: usize = 16;
    pub const HEIGHT: usize = 16;
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            pixels: [[Vec4::new(255, 255, 255, 255); 16]; 16],
        }
    }
}

pub struct Sprite {
    pos: Vec2,
    tiles: Vec<Vec<Tile>>,
}

