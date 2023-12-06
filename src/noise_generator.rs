use crate::tile_data::TileData;
use bevy::prelude::*;
use noise::NoiseFn;

const SCALE: f64 = 0.035;

#[derive(Resource)]
pub struct NoiseGenerator {
    generator: noise::OpenSimplex,
}
impl NoiseGenerator {
    pub fn new(seed: u32) -> Self {
        NoiseGenerator {
            generator: noise::OpenSimplex::new(seed),
        }
    }

    pub fn get_tile_data(&self, chunk_pos: IVec2, tile_pos_x: u32, tile_pos_y: u32) -> TileData {
        let x = (chunk_pos.x as f64 * crate::game_map::CHUNK_SIZE.x as f64) + tile_pos_x as f64;
        let y = (chunk_pos.y as f64 * crate::game_map::CHUNK_SIZE.y as f64) + tile_pos_y as f64;

        let point = [x * SCALE, y * SCALE];

        TileData {
            height: self.generator.get(point) as f32,
        }
    }
}
