use crate::tile_data::TileData;
use bevy::prelude::*;
use noise::{MultiFractal, NoiseFn};

#[derive(Resource)]
pub struct NoiseGenerator {
    height: noise::OpenSimplex,
    biome: noise::BasicMulti<noise::OpenSimplex>,
}
impl NoiseGenerator {
    pub fn new(seed: u32) -> Self {
        NoiseGenerator {
            height: noise::OpenSimplex::new(seed),
            biome: noise::BasicMulti::new(seed).set_frequency(5.0),
        }
    }

    pub fn get_tile_data(&self, chunk_pos: IVec2, tile_pos_x: u32, tile_pos_y: u32) -> TileData {
        let x = (chunk_pos.x as f64 * crate::game_map::CHUNK_SIZE.x as f64) + tile_pos_x as f64;
        let y = (chunk_pos.y as f64 * crate::game_map::CHUNK_SIZE.y as f64) + tile_pos_y as f64;

        TileData {
            height: self.height.get(Self::get_point(x, y, 0.035)) as f32,
            biome: self.biome.get(Self::get_point(x, y, 0.022)) as f32,
        }
    }

    fn get_point(x: f64, y: f64, resolution: f64) -> [f64; 2] {
        [x * resolution, y * resolution]
    }
}
