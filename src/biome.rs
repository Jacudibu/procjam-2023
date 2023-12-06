use crate::tile_data::TileType;

pub struct Biome {
    deep_water: f32,
    water: f32,
    sand: f32,
    grass: f32,
}

pub const WET: Biome = Biome {
    deep_water: -0.1,
    water: 0.1,
    sand: 0.3,
    grass: 0.3,
};
pub const REGULAR: Biome = Biome {
    deep_water: -0.4,
    water: -0.3,
    sand: -0.2,
    grass: 0.4,
};
pub const DRY: Biome = Biome {
    deep_water: -0.6,
    water: -0.5,
    sand: 0.2,
    grass: 0.2,
};

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
