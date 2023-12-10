mod biome;
mod camera;
mod game;
mod game_map;
mod game_ui;
mod noise_generator;
mod tile_data;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin,
};

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(game::GamePlugin)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(ScreenEntityDiagnosticsPlugin)
        .run();
}
