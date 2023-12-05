use bevy::math::IVec2;
use bevy::prelude::{Color, Resource};
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

    fn get_01(&self, x: f64, y: f64) -> f32 {
        ((self.generator.get([x * SCALE, y * SCALE]) + 1.0) * 0.5) as f32
    }

    fn get(&self, chunk_pos: IVec2, tile_pos_x: u32, tile_pos_y: u32) -> f32 {
        let x = (chunk_pos.x as f64 * crate::game_map::CHUNK_SIZE.x as f64) + tile_pos_x as f64;
        let y = (chunk_pos.y as f64 * crate::game_map::CHUNK_SIZE.y as f64) + tile_pos_y as f64;

        self.generator.get([x * SCALE, y * SCALE]) as f32
    }

    pub fn get_color(&self, chunk_pos: IVec2, tile_pos_x: u32, tile_pos_y: u32) -> Color {
        let x = (chunk_pos.x as f64 * crate::game_map::CHUNK_SIZE.x as f64) + tile_pos_x as f64;
        let y = (chunk_pos.y as f64 * crate::game_map::CHUNK_SIZE.y as f64) + tile_pos_y as f64;

        Color::rgb(
            0.0,
            //self.get_01(x, y, 0.0),
            self.get_01(x, y),
            0.0, //self.get_01(x, y, 2.0),
        )
    }
}
