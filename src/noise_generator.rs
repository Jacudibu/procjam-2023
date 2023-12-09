use crate::tile_data::TileData;
use bevy::prelude::*;
use noise::{MultiFractal, NoiseFn, Seedable};

#[derive(Resource)]
pub struct NoiseGenerator {
    pub values: NoiseValues,

    height: noise::OpenSimplex,
    biome: noise::BasicMulti<noise::OpenSimplex>,
}

impl PartialEq for NoiseGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values && self.height.seed() == other.height.seed()
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct NoiseValues {
    pub resolution: f64,
}
impl Default for NoiseValues {
    fn default() -> Self {
        NoiseValues { resolution: 0.035 }
    }
}

impl Default for NoiseGenerator {
    fn default() -> Self {
        NoiseGenerator::new(&String::from("42"), NoiseValues::default())
    }
}

impl NoiseGenerator {
    pub fn new(seed: &String, values: NoiseValues) -> Self {
        let seed = seed.reflect_hash().unwrap_or(42) as u32;
        NoiseGenerator {
            values,
            height: noise::OpenSimplex::new(seed),
            biome: noise::BasicMulti::new(seed).set_frequency(5.0),
        }
    }

    pub fn get_tile_data(&self, chunk_pos: IVec2, tile_pos_x: u32, tile_pos_y: u32) -> TileData {
        let x = (chunk_pos.x as f64 * crate::game_map::CHUNK_SIZE.x as f64) + tile_pos_x as f64;
        let y = (chunk_pos.y as f64 * crate::game_map::CHUNK_SIZE.y as f64) + tile_pos_y as f64;

        TileData {
            height: self
                .height
                .get(Self::get_point(x, y, self.values.resolution)) as f32,
            humidity: self.biome.get(Self::get_point(x, y, 0.022)) as f32,
        }
    }

    fn get_point(x: f64, y: f64, resolution: f64) -> [f64; 2] {
        [x * resolution, y * resolution]
    }
}
