use bracket_lib::prelude::*;
use specs::prelude::*;
use specs::Component;

#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    /// Creates a new map with empty floor tiles. Size of 80x50 (W x H)
    pub fn new_base_map() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Floor; 80*50],
            width: 80,
            height: 50,
        };

        map
    }
}

pub fn draw_map(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for tile in map.tiles.iter(){
        let glyph;
        let fg;
        match tile {
            &TileType::Floor => {
                glyph = to_cp437(' ');
                fg = RGB::from(GREY);
            }
            &TileType::Wall => {
                glyph = to_cp437('#');
                fg = RGB::from_f32(0.0, 1.0, 0.0);
            }
        }
        ctx.set(x, y, fg, RGB::from_f32(0.0, 0.0, 0.0), glyph);

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}