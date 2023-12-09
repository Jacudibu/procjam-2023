use crate::game_map::{ChunkData, HighlightedTile, CHUNK_SIZE};
use crate::noise_generator::{NoiseGenerator, NoiseValues};
use crate::tile_data::TileData;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::egui::{Align2, Grid, Pos2, Widget};
use bevy_egui::*;

pub struct GameUIPlugin;
impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, ui_system)
            .add_event::<RegenerateMapEvent>()
            .insert_resource(UnappliedSettings {
                map_generator_values: NoiseValues::default(),
            });
    }
}

#[derive(Resource)]
pub struct UnappliedSettings {
    pub map_generator_values: NoiseValues,
}

#[derive(Event)]
pub struct RegenerateMapEvent;

fn ui_system(
    mut contexts: EguiContexts,
    tile_query: Query<(&TilePos, &TilemapId, &TileData), With<HighlightedTile>>,
    tilemap_query: Query<(Entity, &ChunkData)>,
    mut map_gen: ResMut<NoiseGenerator>,
    mut unapplied_settings: ResMut<UnappliedSettings>,
    mut regenerate_map_event: EventWriter<RegenerateMapEvent>,
) {
    if let Ok((tile_pos, tilemap_id, tile_data)) = tile_query.get_single() {
        if let Ok((_, chunk_data)) = tilemap_query.get(tilemap_id.0) {
            egui::Window::new(format!(
                "{} | {}",
                tile_pos.x as i32 + chunk_data.position.x * CHUNK_SIZE.x as i32,
                tile_pos.y as i32 + chunk_data.position.y * CHUNK_SIZE.y as i32
            ))
            .collapsible(false)
            .resizable(false)
            .fixed_pos(Pos2::new(5.0, 5.0))
            .show(contexts.ctx_mut(), |ui| {
                ui.label(tile_data.to_string());
                ui.separator();
                ui.heading("Chunk Data");
                ui.label(format!("Local: x: {} | y: {}", tile_pos.x, tile_pos.y));
                ui.label(format!(
                    "Chunk: x: {} | y: {}",
                    chunk_data.position.x, chunk_data.position.y
                ));
            });
        }
    }

    let mut apply = false;
    egui::Window::new("Settings")
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::LEFT_BOTTOM, egui::Vec2::new(0.0, 0.0))
        .fixed_pos(Pos2::new(5.0, 5.0))
        .show(contexts.ctx_mut(), |ui| {
            Grid::new("table").show(ui, |ui| {
                ui.label("Biome Resolution:");
                egui::Slider::new(
                    &mut unapplied_settings.map_generator_values.resolution,
                    0.001..=0.1,
                )
                .ui(ui);
                ui.end_row();
                apply = ui.button("Apply").clicked();
            })
        });

    if apply {
        map_gen.values = unapplied_settings.map_generator_values;
        regenerate_map_event.send(RegenerateMapEvent);
    }
}
