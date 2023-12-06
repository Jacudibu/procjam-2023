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
    pub biome: f32,
}
impl TileData {
    pub fn get_tile_type(&self) -> TileType {
        let biome = Biome {
            deep_water: -0.4,
            water: -0.3,
            sand: -0.1,
            grass: 0.4,
        };

        return biome.evaluate(self.height);
    }

    pub fn get_color(&self) -> Color {
        Color::rgb(0.0, (self.height + 1.0) * 0.5, 0.0)
    }
}
impl std::fmt::Display for TileData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Height: {}\nBiome: {}\n-> {:?}",
            self.height,
            self.biome,
            self.get_tile_type()
        )
    }
}

pub struct Biome {
    deep_water: f32,
    water: f32,
    sand: f32,
    grass: f32,
}
impl Biome {
    pub fn evaluate(&self, height: f32) -> TileType {
        if height < self.deep_water {
            return TileType::DeepWater;
        }
        if height < self.water {
            return TileType::Water;
        }
        if height < self.sand {
            return TileType::Sand;
        }
        if height < self.grass {
            return TileType::Grass;
        }

        return TileType::Stone;
    }
}
