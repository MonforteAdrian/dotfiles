use std::ops::Neg;

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
    window::PrimaryWindow,
};
use glam::uvec2;
use isometric::{algorithms::range_fov, *};

const TILE_SIZE: Vec2 = Vec2::splat(16.0);
const CHUNK_SIZE: Vec2 = Vec2::new(TILE_SIZE.x * 12.0, TILE_SIZE.y * 3.0);

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<HighlightedTiles>()
        .add_systems(Startup, (setup_camera, setup_grid))
        .add_systems(Update, handle_input)
        .run();
}

#[derive(Debug, Default, Resource)]
struct HighlightedTiles {
    pub selected: Iso,
}

#[derive(Debug, Resource)]
struct IsoGrid {
    pub entities: HashMap<Iso, Entity>,
    pub layout: IsoLayout,
    pub blocked_coords: HashSet<Iso>,
    pub visible_entities: HashSet<Entity>,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_grid(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    window: Query<&mut Window>,
) {
    // We get the window dimensions and calculate the numnber of tiles that would fit width and height adding a bit of room outside
    let window = window.single();
    let chunks_wide = (window.width() / CHUNK_SIZE.x).ceil();
    let chunks_height = (window.height() / CHUNK_SIZE.y).ceil();
    let cols = (chunks_wide / 2.0) as i32;
    let rows = (chunks_height / 2.0) as i32;

    // Load textures
    let texture = asset_server.load("textures/the_four_horsemen_of_the_apocalypse.png");
    let atlas_layout = TextureAtlasLayout::from_grid(uvec2(32, 32), 2, 2, None, None);
    let atlas_layout = atlas_layouts.add(atlas_layout);

    let offset_layers = TILE_SIZE.y / 2.0 * 3.0;
    let offset_center_tile = TILE_SIZE.y / 4.0;
    // Create the Layout
    let layout = IsoLayout {
        tile_size: TILE_SIZE,
        origin: Vec3::new(0., -(offset_layers + offset_center_tile), 0.),
        ..default()
    };

    // Generate the tiles and spawn them
    let mut blocked_coords = HashSet::new();
    let chunks = mapping::generate_mesh_of_chunks(cols, cols.neg(), rows, rows.neg());
    let entities = mapping::get_sorted_tiles(chunks)
        .into_iter()
        .enumerate()
        .map(|(i, tile)| {
            let pos = layout.tile_to_world_pos(tile);

            let index = 0;
            //let index = if i % 11 == 0 {
            //    blocked_coords.insert(tile);
            //    1
            //} else {
            //    0
            //};
            let entity = commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(TILE_SIZE),
                            ..default()
                        },
                        texture: texture.clone(),
                        transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                        ..default()
                    },
                    TextureAtlas {
                        index,
                        layout: atlas_layout.clone(),
                    },
                ))
                .id();
            (tile, entity)
        })
        .collect();
    commands.insert_resource(IsoGrid {
        entities,
        layout,
        blocked_coords,
        visible_entities: Default::default(),
    });
}

// Input interaction
fn handle_input(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut tiles: Query<&mut TextureAtlas>,
    mut current: Local<Iso>,
    mut grid: ResMut<IsoGrid>,
) {
    // Get the cursor position
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p))
    {
        // Translate the cursor position to tile position
        let all_iso_pos_v = grid.layout.world_pos_to_tile(pos);
        let iso_pos_v: Vec<Iso> = all_iso_pos_v
            .into_iter()
            .filter(|tile| grid.entities.get(tile).copied().is_some())
            .collect();
        let Some(tile_pos) = iso_pos_v.last() else {
            return;
        };
        let Some(entity) = grid.entities.get(tile_pos).copied() else {
            return;
        };
        // Retrieve the texture for the tile
        let Ok(mut atlas) = tiles.get_mut(entity) else {
            return;
        };
        // Block/Unblock on click
        if buttons.just_pressed(MouseButton::Left) {
            if grid.blocked_coords.contains(tile_pos) {
                grid.blocked_coords.remove(tile_pos);
                atlas.index = 0;
            } else {
                grid.blocked_coords.insert(*tile_pos);
                grid.visible_entities.remove(&entity);
                atlas.index = 1;
            }
            return;
        }
        if *tile_pos == *current {
            return;
        }
        *current = *tile_pos;
        for entity in &grid.visible_entities {
            let Ok(mut atlas) = tiles.get_mut(*entity) else {
                return;
            };
            atlas.index = 0;
        }
        let fov = range_fov(*tile_pos, (tile_pos.y + 50) as u32, |h| {
            grid.blocked_coords.contains(&h)
        });
        let entities: HashSet<_> = fov
            .into_iter()
            .filter_map(|h| grid.entities.get(&h).copied())
            .collect();
        for entity in &entities {
            let Ok(mut atlas) = tiles.get_mut(*entity) else {
                return;
            };
            atlas.index = 2;
        }
        grid.visible_entities = entities;
    }
}
