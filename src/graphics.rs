use alloc::vec::Vec;
use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics::Pixel;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::pixelcolor::raw::ToBytes;
use embedded_graphics::prelude::Point;
use tinybmp::{Bmp, Pixels};

use uefi::CStr16;
use uefi_services::println;

use crate::math::{Vec2, Color4};
use crate::file_loader::{FileLoader};

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
    pub pixels: [[Color4; 16]; 16],
}

impl Tile {
    pub const WIDTH: usize = 16;
    pub const HEIGHT: usize = 16;
    fn new_from_pixels(pixels: [[Color4; 16]; 16]) -> Self {
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
        let width_in_tiles = bmp.size().width / 16;
        let height_in_tiles = bmp.size().height / 16;
        for tile_x in 0..width_in_tiles {
            for tile_y in 0..height_in_tiles {
                let mut tile_bitmap = [[Color4::new(255, 255, 255, 255); 16]; 16];
                for x in 0..16 {
                    for y in 0..16 {
                        let point = Point::new((tile_x * 16 + x) as i32, (tile_y * 16 + y) as i32);
                        let pixel = bmp.pixel(point).unwrap();
                        let bytes = pixel.to_le_bytes();
                        let alpha = if bytes[0] == 255 && bytes[1] == 255 && bytes[2] == 255 {
                            0
                        } else {
                            1
                        };
                        let color = Color4::new(bytes[0] as i32, bytes[1] as i32, bytes[2] as i32, alpha);
                        tile_bitmap[x as usize][y as usize] = color;
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

#[derive(Clone)]
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

impl Sprite {
    pub fn new(entities: Vec<Vec<Entity>>) -> Self {
        Sprite {
            pos: Vec2::new(0, 0),
            entities: entities,
        }
    }

    fn tiles_width(&self) -> usize {
        self.entities.len()
    }

    fn tiles_height(&self) -> usize {
        self.entities[0].len()
    }
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

pub struct Level {
    pub sprite: Sprite,
}

impl Level {
    pub const WIDTH: usize = 40;
    pub const HEIGHT: usize = 30;

    pub fn new_from_name(file_loader: &FileLoader, level_name: &str) -> Self {
        let level_file_name = format!("{}.lvl", level_name);
        let level_bytes = file_loader.read_file(&level_file_name, Some("levels")).unwrap();

        let mut entities = vec![vec![]];

        for x in 0..Self::WIDTH {
            entities.push(vec![]);

            for y in 0..Self::HEIGHT {
                let entity_id_char: char = level_bytes[y * Self::WIDTH + x].into();
                let entity_id = format!("{}", entity_id_char);

                //let field_entity = Entity::new_from_id(&file_loader, entity_id);
                let field_entity = Entity::default();
                entities[x].push(field_entity);
            }
        }

        Level {
            sprite: Sprite::new(entities),
        }
    }

    pub fn collides(&self, sprite: &Sprite, move_dir: Vec2) -> Option<Vec<Entity>> {
        let moved_pos = sprite.pos + move_dir;
        let index_pos = moved_pos - self.sprite.pos;

        let (first_x, first_y) = (index_pos[0] / (Tile::WIDTH as i32), index_pos[1] / (Tile::HEIGHT as i32));
        let (end_x, end_y) = (first_x + (sprite.tiles_width() as i32) + 1, first_y + (sprite.tiles_height() as i32) + 1);

        let mut collision_entities = vec![];
        for x in first_x..end_x {
            for y in first_y..end_y {
                let entity = self.sprite.entities[x as usize][y as usize].clone();
                if entity.wall {
                    let collides = sprite.pos[0] < self.sprite.pos[0] + (x + 1) * (Tile::WIDTH as i32) ||
                        sprite.pos[0] + (sprite.width() as i32) > self.sprite.pos[0] + x * (Tile::WIDTH as i32) ||
                        sprite.pos[1] < self.sprite.pos[1] + (y + 1) * (Tile::HEIGHT as i32) ||
                        sprite.pos[1] + (sprite.height() as i32) > self.sprite.pos[1] + y * (Tile::HEIGHT as i32);
                    if collides {
                        collision_entities.push(entity);
                    }
                }
            }
        }

        if collision_entities.len() == 0 {
            None
        } else {
            Some(collision_entities)
        }
    }
}
