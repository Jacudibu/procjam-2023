use crate::game_map::{ChunkData, HighlightedTile, CHUNK_SIZE};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::*;

pub struct GameUIPlugin;
impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin).add_systems(Update, ui_system);
    }
}

fn ui_system(
    mut contexts: EguiContexts,
    tile_query: Query<(&TilePos, &TilemapId), With<HighlightedTile>>,
    tilemap_query: Query<(Entity, &ChunkData)>,
) {
    if let Ok((tile_pos, tilemap_id)) = tile_query.get_single() {
        if let Ok((_, chunk_data)) = tilemap_query.get(tilemap_id.0) {
            egui::Window::new("Tile Info").show(contexts.ctx_mut(), |ui| {
                ui.label(format!(
                    "{} | {}",
                    tile_pos.x as i32 + chunk_data.position.x * CHUNK_SIZE.x as i32,
                    tile_pos.y as i32 + chunk_data.position.y * CHUNK_SIZE.y as i32
                ));
                ui.label(format!("Local: x: {} | y: {}", tile_pos.x, tile_pos.y));
                ui.label(format!(
                    "Chunk: x: {} | y: {}",
                    chunk_data.position.x, chunk_data.position.y
                ));
            });
        }
    }
}
