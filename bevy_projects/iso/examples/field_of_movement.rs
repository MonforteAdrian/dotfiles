use std::ops::Neg;

use bevy::{prelude::*, utils::HashSet, window::PrimaryWindow};
use glam::vec2;
use isometric::{algorithms::field_of_movement, storage::IsometricMap, *};
use rand::prelude::*;

/// World size of the hexagons (outer radius)
const TILE_SIZE: Vec2 = Vec2::splat(14.0);
const CHUNK_SIZE: Vec2 = Vec2::new(TILE_SIZE.x * 12.0, TILE_SIZE.y * 3.0);
const BUDGET: u32 = 10;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_grid))
        //.add_systems(Update, handle_input)
        .run();
}

type Cost = Option<u32>;

#[derive(Debug, Resource)]
struct IsoGrid {
    pub entities: IsometricMap<(Cost, Entity)>,
    pub reachable_entities: HashSet<Entity>,
    pub layout: IsoLayout,
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Input interaction
fn handle_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut tile_transforms: Query<(Entity, &mut Transform)>,
    mut current: Local<Iso>,
    mut grid: ResMut<IsoGrid>,
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p))
    {
        let tile_pos = grid.layout.world_pos_to_tile(pos);

        if tile_pos == *current {
            return;
        }
        *current = tile_pos;

        let field_of_movement =
            field_of_movement(tile_pos, BUDGET, |h| grid.entities.get(h).and_then(|c| c.0));

        let reachable_entities: HashSet<_> = field_of_movement
            .into_iter()
            .filter_map(|h| grid.entities.get(h).map(|&(_, ent)| ent))
            .collect();
        for (entity, mut transform) in tile_transforms.iter_mut() {
            if reachable_entities.contains(&entity) {
                *transform = transform.with_scale(Vec3::splat(0.9));
            } else {
                *transform = transform.with_scale(Vec3::splat(1.));
            }
        }

        grid.reachable_entities = reachable_entities;
    }
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
    let atlas_layout = TextureAtlasLayout::from_grid(vec2(32.0, 32.0), 2, 2, None, None);
    let atlas_layout = atlas_layouts.add(atlas_layout);

    let offset_layers = TILE_SIZE.y / 2.0 * 3.0;
    let offset_center_tile = TILE_SIZE.y / 4.0;
    // Create the Layout
    let layout = IsoLayout {
        tile_size: TILE_SIZE,
        origin: Vec3::new(0., -(offset_layers + offset_center_tile), 0.),
        ..default()
    };

    let mut rng = rand::thread_rng();

    let chunks = mapping::generate_mesh_of_chunks(cols, cols.neg(), rows, rows.neg());
    let entities = mapping::get_sorted_tiles(chunks).into_iter().map(|tile| {
        let cost = rng.gen_range(0..=3);
        let pos = layout.tile_to_world_pos(tile);
        let index = cost;
        let cost = if (0..3).contains(&cost) {
            Some(cost)
        } else {
            None
        };
        let entity = commands
            .spawn(SpriteSheetBundle {
                sprite: Sprite {
                    custom_size: Some(TILE_SIZE),
                    ..default()
                },
                texture: texture.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                atlas: TextureAtlas {
                    index,
                    layout: atlas_layout.clone(),
                },
                ..default()
            })
            .id();
        (cost, entity)
    });
    commands.insert_resource(IsoGrid {
        entities,
        reachable_entities: Default::default(),
        layout,
    })
}
