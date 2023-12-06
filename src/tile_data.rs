use bevy::prelude::*;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum TileType {
    DeepWater,
    Water,
    Sand,
    Grass,
    Stone,
}

#[derive(Component)]
pub struct TileData {
    pub height: f32,
}
impl TileData {
    pub fn get_tile_type(&self) -> TileType {
        if self.height < -0.4 {
            return TileType::DeepWater;
        }
        if self.height < -0.3 {
            return TileType::Water;
        }
        if self.height < -0.1 {
            return TileType::Sand;
        }
        if self.height < 0.4 {
            return TileType::Grass;
        }

        return TileType::Stone;
    }

    pub fn get_color(&self) -> Color {
        Color::rgb(0.0, (self.height + 1.0) * 0.5, 0.0)
    }
}
impl std::fmt::Display for TileData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Height: {}\n-> {:?}", self.height, self.get_tile_type())
    }
}
