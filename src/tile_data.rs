use crate::biome;
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
    pub humidity: f32,
}
impl TileData {
    pub fn get_tile_type(&self) -> TileType {
        if self.humidity < 0.0 {
            biome::REGULAR.evaluate_multibiome(&biome::WET, self.height, 1.0 + self.humidity)
        } else {
            biome::REGULAR.evaluate_multibiome(&biome::DRY, self.height, 1.0 - self.humidity)
        }
    }

    pub fn get_color(&self) -> Color {
        Color::rgb(0.0, (self.height + 1.0) * 0.5, 0.0)
    }
}
impl std::fmt::Display for TileData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Height: {}\nHumidity: {}\n-> {:?}",
            self.height,
            self.humidity,
            self.get_tile_type()
        )
    }
}
