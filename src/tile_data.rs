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
        let ocean = Biome {
            deep_water: -0.1,
            water: 0.1,
            sand: 0.3,
            grass: 0.3,
        };

        let forest = Biome {
            deep_water: -0.4,
            water: -0.3,
            sand: -0.2,
            grass: 0.4,
        };

        let desert = Biome {
            deep_water: -0.6,
            water: -0.5,
            sand: 0.2,
            grass: 0.2,
        };

        if self.biome < 0.0 {
            forest.evaluate_multibiome(&ocean, self.height, 1.0 + self.biome)
        } else {
            forest.evaluate_multibiome(&desert, self.height, 1.0 - self.biome)
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

    pub fn evaluate_multibiome(&self, other: &Self, height: f32, strength_this: f32) -> TileType {
        if height < self.deep_water * strength_this + other.deep_water * (1.0 - strength_this) {
            return TileType::DeepWater;
        }
        if height < self.water * strength_this + other.water * (1.0 - strength_this) {
            return TileType::Water;
        }
        if height < self.sand * strength_this + other.sand * (1.0 - strength_this) {
            return TileType::Sand;
        }
        if height < self.grass * strength_this + other.grass * (1.0 - strength_this) {
            return TileType::Grass;
        }

        TileType::Stone
    }
}
