use std::ops::Neg;

use bevy::{prelude::*, utils::HashMap, window::PrimaryWindow};
use glam::uvec2;
use isometric::*;
use mapping::CHUNK_DIMENSIONS;

const TILE_SIZE: Vec2 = Vec2::splat(64.0);
const CHUNK_SIZE: Vec2 = Vec2::new(
    TILE_SIZE.x * CHUNK_DIMENSIONS.0 as f32,
    TILE_SIZE.y * (CHUNK_DIMENSIONS.1 as f32 / 4.0),
);

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<SelectedTile>()
        .add_systems(Startup, (setup_camera, setup_grid))
        .add_systems(Update, handle_input)
        .run();
}

#[derive(Debug, Default, Resource)]
struct SelectedTile {
    pub selected: Option<Iso>,
}

#[derive(Debug, Resource)]
struct IsoGrid {
    pub entities: HashMap<Iso, Entity>,
    pub layout: IsoLayout,
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
    let chunks = mapping::generate_mesh_of_chunks(cols, cols.neg(), rows, rows.neg());
    let entities = mapping::get_sorted_tiles(chunks)
        .into_iter()
        .map(|tile| {
            let pos = layout.tile_to_world_pos(tile);
            let index = 0;
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
                .with_children(|b| {
                    if tile.z == layout.top_layer - 1 {
                        b.spawn(Text2dBundle {
                            text: Text::from_section(
                                format!("{},{},{}", tile.x, tile.y, tile.z),
                                TextStyle {
                                    font_size: 10.0,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            ),
                            transform: Transform::from_xyz(0.0, 16.0, 10.0),
                            ..default()
                        });
                    }
                })
                .id();
            (tile, entity)
        })
        .collect();
    commands.insert_resource(IsoGrid { entities, layout });
}

// Input interaction
fn handle_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<IsoGrid>,
    mut tiles: Query<&mut TextureAtlas>,
    mut selected_tile: ResMut<SelectedTile>,
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
        // Check if there is a tile where the cursor is. If there are several of them pick the top one
        let iso_pos_v: Vec<Iso> = all_iso_pos_v
            .into_iter()
            .filter(|tile| grid.entities.get(tile).copied().is_some())
            .collect();
        let Some(iso_pos) = iso_pos_v.last() else {
            return;
        };
        let Some(entity) = grid.entities.get(iso_pos).copied() else {
            return;
        };
        // Clean previous selected tile
        if let Some(selected_entity) = selected_tile.selected {
            if *iso_pos != selected_entity {
                if let Ok(mut atlas2) = tiles.get_mut(grid.entities[&selected_entity]) {
                    atlas2.index = 0;
                } else {
                    return;
                };
            }
        }
        // Retrieve the texture for the tile
        let Ok(mut atlas) = tiles.get_mut(entity) else {
            return;
        };
        // Mark the tile as selected
        selected_tile.selected = Some(*iso_pos);
        atlas.index = 3;
    }
}
