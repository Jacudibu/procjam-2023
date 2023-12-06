use crate::game::CursorPos;
use crate::noise_generator::NoiseGenerator;
use crate::tile_data::TileType;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use std::collections::HashSet;

// Right now mostly sticking to the example code found at https://github.com/divark/bevy_ecs_tilemap/blob/0.12-fixes/examples/

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };
pub const CHUNK_SIZE: UVec2 = UVec2 { x: 32, y: 32 };
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

pub struct GameMapPlugin;
impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::default())
            .insert_resource(TilemapRenderSettings {
                render_chunk_size: RENDER_CHUNK_SIZE,
                ..Default::default()
            })
            .insert_resource(NoiseGenerator::new(42))
            .add_plugins(TilemapPlugin)
            .add_systems(Update, spawn_chunks_around_camera)
            .add_systems(Update, despawn_out_of_range_chunks)
            .add_systems(Update, highlight_tile_below_cursor);
    }
}

#[derive(Component)]
pub struct HighlightedTile;
#[derive(Component)]
pub struct ChunkData {
    pub position: IVec2,
}

fn tile_type_to_texture_index(tile_type: &TileType) -> TileTextureIndex {
    match tile_type {
        TileType::Water => TileTextureIndex(0),
        TileType::Sand => TileTextureIndex(1),
        TileType::Grass => TileTextureIndex(2),
        TileType::Stone => TileTextureIndex(3),
        TileType::DeepWater => TileTextureIndex(4),
    }
}

fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &AssetServer,
    chunk_pos: IVec2,
    noise: &Res<NoiseGenerator>,
) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());

    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let tile_pos = TilePos { x, y };
            let tile_data = noise.get_tile_data(chunk_pos, x, y);
            let tile_type = tile_data.get_tile_type();
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: tile_type_to_texture_index(&tile_type),
                    ..Default::default()
                })
                .insert(tile_data)
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
        0.0,
    ));

    let tile_texture: Handle<Image> = asset_server.load("sprites/tiles.png");
    commands
        .entity(tilemap_entity)
        .insert(TilemapBundle {
            grid_size: TILE_SIZE.into(),
            size: CHUNK_SIZE.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(tile_texture),
            tile_size: TILE_SIZE,
            transform,
            ..Default::default()
        })
        .insert(ChunkData {
            position: chunk_pos,
        });
}

#[derive(Default, Debug, Resource)]
struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}

fn calculate_ideal_chunk_spawn_distance(camera_rect: &Rect) -> IVec2 {
    IVec2::new(
        (camera_rect.width() as i32 / (CHUNK_SIZE.x as i32 * TILE_SIZE.x as i32)) + 1,
        (camera_rect.height() as i32 / (CHUNK_SIZE.y as i32 * TILE_SIZE.y as i32)) + 1,
    )
}

fn spawn_chunks_around_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    mut chunk_manager: ResMut<ChunkManager>,
    noise: Res<NoiseGenerator>,
) {
    for (transform, projection) in camera_query.iter() {
        let chunk_spawn_distance = calculate_ideal_chunk_spawn_distance(&projection.area);
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        for y in (camera_chunk_pos.y - chunk_spawn_distance.y)
            ..(camera_chunk_pos.y + chunk_spawn_distance.y)
        {
            for x in (camera_chunk_pos.x - chunk_spawn_distance.x)
                ..(camera_chunk_pos.x + chunk_spawn_distance.x)
            {
                let chunk = IVec2::new(x, y);
                if !chunk_manager.spawned_chunks.contains(&chunk) {
                    info!("Spawning chunk {}", &chunk);
                    chunk_manager.spawned_chunks.insert(chunk);
                    spawn_chunk(&mut commands, &asset_server, chunk, &noise);
                    return;
                }
            }
        }
    }
}

fn despawn_out_of_range_chunks(
    mut commands: Commands,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    chunks_query: Query<(Entity, &ChunkData)>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for (camera_transform, projection) in camera_query.iter() {
        let chunk_spawn_distance = calculate_ideal_chunk_spawn_distance(&projection.area);
        let camera_chunk_pos = camera_pos_to_chunk_pos(&camera_transform.translation.xy());
        for (entity, chunk_data) in chunks_query.iter() {
            let chunk_pos = chunk_data.position;
            let distance = IVec2::new(
                (chunk_pos.x - camera_chunk_pos.x).abs(),
                (chunk_pos.y - camera_chunk_pos.y).abs(),
            );
            if distance.x > chunk_spawn_distance.x + 1 || distance.y > chunk_spawn_distance.y + 1 {
                info!("Despawning chunk {}", &chunk_pos);
                chunk_manager.spawned_chunks.remove(&chunk_pos);
                commands.entity(entity).despawn_recursive();
                return;
            }
        }
    }
}

fn highlight_tile_below_cursor(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
    highlighted_tiles_q: Query<Entity, With<HighlightedTile>>,
) {
    // Un-highlight any previously highlighted tile labels.
    for entity in highlighted_tiles_q.iter() {
        commands.entity(entity).remove::<HighlightedTile>();
    }

    let cursor_pos: Vec4 = Vec4::from((cursor_pos.world, 0.0, 1.0));
    for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {
        // We need to make sure that the cursor's world position is correct relative to the map
        // due to any map transformation.
        let cursor_pos_relative_to_tilemap: Vec2 = {
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };
        // Once we have a world position we can transform it into a possible tile position.
        if let Some(tile_pos) = TilePos::from_world_pos(
            &cursor_pos_relative_to_tilemap,
            map_size,
            grid_size,
            map_type,
        ) {
            // Highlight the relevant tile's label
            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                commands.entity(tile_entity).insert(HighlightedTile);
            }
        }
    }
}
