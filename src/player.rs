use crate::{
    animation::{AnimationBundle, AnimationTimer, Animations, CurrentAnimation},
    helpers::coordinate_utils::{tiled_top_left, CoordinateOps},
    levels::{Level, LevelConfig},
};
use bevy::{input::mouse::MouseWheel, math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::TilePos,
};
use std::f32::consts::PI;

#[derive(Component)]
pub struct Health {
    current: usize,
    max: usize,
}

impl Health {
    fn new(max: usize) -> Self {
        Self {
            current: max,
            max: max,
        }
    }

    fn damage(&self, damage: usize) -> Self {
        Self {
            current: if self.current < damage {
                0
            } else {
                self.current - damage
            },
            max: self.max,
        }
    }
}

impl Default for Health {
    fn default() -> Self {
        Health::new(100)
    }
}

#[derive(Default, Component)]
pub struct Player;

const IDLE: &str = "idle";
const RUN: &str = "run";

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    pub _marker: Player,
    pub health: Health,
    pub sprite: SpriteSheetBundle,
    pub animations: AnimationBundle,
    pub animation_timer: AnimationTimer,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}

fn movement(
    mut mouse_input: EventReader<MouseWheel>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: Query<
        (
            &mut CurrentAnimation,
            &Animations,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &mut Transform,
        ),
        (With<Player>, Without<Camera>),
    >,
    level: Query<&Level>,
    tilemap: Query<
        (&TilemapGridSize, &TilemapType, &TilemapSize, &Transform),
        (Without<Player>, Without<Camera>),
    >,
    level_config_assets: Res<Assets<LevelConfig>>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    // Camera
    for (mut camera_transform, mut ortho) in camera.iter_mut() {
        for (mut current_animation, animations, mut timer, mut sprite, mut player_transform) in
            player.iter_mut()
        {
            zoom_handler(&mut mouse_input, &mut ortho);

            let z = player_transform.translation.z;
            let direction = direction_from(&keyboard_input);

            if direction.x > 0. {
                player_transform.rotation = Quat::from_rotation_y(0.);
            } else if direction.x < 0. {
                player_transform.rotation = Quat::from_rotation_y(PI);
            }

            // idle / run
            if direction.length() == 0. {
                current_animation.change(&animations.get(IDLE), &mut sprite, &mut timer);
            } else {
                current_animation.change(&animations.get(RUN), &mut sprite, &mut timer);
            }

            let change = time.delta_seconds() * direction * 500.;
            let mut changed_pos = player_transform.translation + change;

            if is_hitting_obstacle(&changed_pos, &level, &tilemap, &level_config_assets) {
                changed_pos = player_transform.translation;
            }

            if change != Vec3::ZERO {
                player_transform.translation = changed_pos;
                // Important! We need to restore the Z values when moving the camera around.
                // Bevy has a specific camera setup and this can mess with how our layers are shown.
                player_transform.translation.z = z;

                // For now just follow eagerly
                camera_transform.translation = player_transform.translation;
            }
        }
    }
}

// TODO: Move in generic entity controller
fn is_hitting_obstacle(
    entity_translation: &Vec3,
    level: &Query<&Level>,
    tilemap: &Query<
        (&TilemapGridSize, &TilemapType, &TilemapSize, &Transform),
        (Without<Player>, Without<Camera>),
    >,
    level_config_assets: &Res<Assets<LevelConfig>>,
) -> bool {
    let mut is_hit = false;

    level.for_each(|l| {
        tilemap.for_each(|(grid_size, map_type, map_size, map_transform)| {
            if is_hit {
                return;
            }

            let cfg = level_config_assets
                .get(&l.cfg)
                .expect("LevelConfig not found or unexpectedly unloaded!");
            let entity_world_pos = entity_translation.xy();
            let coordinate_zero =
                tiled_top_left(map_transform.translation.xy(), map_size, grid_size);
            let entity_position = entity_world_pos.relative_to(&coordinate_zero).abs();
            let pos = TilePos::from_world_pos(&entity_position, map_size, grid_size, map_type);

            if pos.is_none() {
                warn!(
                    "Can't get tile for position {:?}. Orig: {:?} {:?}",
                    entity_position, coordinate_zero, entity_world_pos
                );
                return;
            }
            let pos = pos.unwrap();

            if !cfg
                .walkable_tiles
                .is_walkable_local(pos.x as usize, pos.y as usize)
            {
                is_hit = true;
            }
        })
    });

    is_hit
}

fn direction_from(keyboard_input: &Res<'_, Input<KeyCode>>) -> Vec3 {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        direction -= Vec3::new(1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction -= Vec3::new(0.0, 1.0, 0.0);
    }

    direction
}

fn zoom_handler(
    mouse_input: &mut EventReader<'_, '_, MouseWheel>,
    ortho: &mut Mut<'_, OrthographicProjection>,
) {
    for ev in mouse_input.iter() {
        if ev.y > 0. {
            ortho.scale += (ev.y / 10.).max(0.1);
        } else if ev.y < 0. {
            ortho.scale += (ev.y / 10.).min(-0.1);
        }
    }

    if ortho.scale > 1.5 {
        ortho.scale = 1.5;
    }

    if ortho.scale < 0.5 {
        ortho.scale = 0.5;
    }
}
