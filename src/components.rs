use bracket_lib::prelude::FontCharType;
use bracket_lib::prelude::RGB;
use specs::prelude::*;
use specs::Component;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Name {
    pub name: String,
}