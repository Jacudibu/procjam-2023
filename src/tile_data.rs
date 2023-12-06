use bevy::prelude::*;
use std::fmt::Formatter;

#[derive(Component)]
pub struct TileData {
    pub height: f32,
}
impl TileData {
    pub fn get_color(&self) -> Color {
        Color::rgb(0.0, (self.height + 1.0) * 0.5, 0.0)
    }
}
impl std::fmt::Display for TileData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Height: {}", self.height)
    }
}
