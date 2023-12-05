mod camera;
mod game;
mod game_map;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin,
};

fn main() {
    App::new()
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
