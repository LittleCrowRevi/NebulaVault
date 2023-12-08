use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::GRID_BOX;

#[derive(Resource)]
pub struct Maps {
    maps: HashMap<String, GridMap>
}

pub struct GridMap {
    size: Option<Vec2>,
    tiles: HashMap<Vec3, Tile>,
    rooms: Vec<Room>,
    name: String,
    
}

impl GridMap {
    fn move_to_unoccupied() {}
}

pub struct Room {
    dim: Rect
}

impl Room {
    fn default_room() -> Self {
        Self {
            dim: Rect::from_corners(vec2(0., 0.), vec2(10., 10.))
        }
    }
}

pub struct Tile {
    tile_type: TileType,
    entity: Option<Entity>
}

pub enum TileType {
    Wall,
    Player,
    Enemy,
    NPC,
    Item,
    Object,
    Void
}

pub fun build() {

}


// Grid
#[derive(Component)]
struct ActiveGrid;

#[derive(Component)]
struct GridBox(Vec3);

#[derive(Component)]
struct Grid(String);

#[derive(Component)]
struct GridPositions(Vec<Vec3>);

fn create_square_grid(
    commands: &mut Commands,
    grid_box: Vec2,
    boundary_box: Vec2,
    grid_name: String,
) -> Entity {
    let grid_x = grid_box.x as usize;
    let grid_y = grid_box.y as usize;

    let mut grid = commands.spawn((Grid(grid_name), SpatialBundle { ..default() }));

    grid.with_children(|parent| {
        for x in ((boundary_box.x - grid_box.x) as i32 / -2..=boundary_box.x as i32 / 2).step_by(grid_x) {
            for y in ((boundary_box.y - grid_box.y) as i32 / -2..=boundary_box.y as i32 / 2).step_by(grid_y) {
                spawn_box(parent, (x) as f32, (y) as f32);
            }
        }
    });

    grid.id()
}

fn spawn_box(commands: &mut ChildBuilder, x: f32, y: f32) {
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(vec3(x, y, 1.0)),
                visibility: Visibility::Visible,
                ..default()
            },
            GridBox(vec3((x / GRID_BOX.x).round(), (y / GRID_BOX.y).round(), 1.0)),
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.2, 0.2, 0.2),
                    custom_size: Some(vec2(GRID_BOX.x, GRID_BOX.y)),
                    ..default()
                },
                ..default()
            });
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.15, 0.15, 0.15),
                    custom_size: Some(vec2(
                        GRID_BOX.x - GRID_BOX.x / GRID_BOX.y,
                        GRID_BOX.y - GRID_BOX.y / GRID_BOX.y,
                    )),
                    ..default()
                },
                ..default()
            });
        });
}