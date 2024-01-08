use bevy::{prelude::*, input::mouse::MouseWheel};
use std::f32::consts::PI;
use crate::animation::{Animations, CurrentAnimation, AnimationTimer, AnimationBundle};

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
            current:  
                if self.current < damage {
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
    pub animation_timer: AnimationTimer
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
    mut player: Query<(
        &mut CurrentAnimation,
        &Animations,
        &mut AnimationTimer, 
        &mut TextureAtlasSprite, 
        &mut Transform
    ), (With<Player>, Without<Camera>)>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    // Camera
    for (mut camera_transform, mut ortho) in camera.iter_mut() {
        for (
            mut current_animation,
            animations,
            mut timer,
            mut sprite,
            mut player_transform
        ) in player.iter_mut() {
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

            player_transform.translation += time.delta_seconds() * direction * 500.;
            // Important! We need to restore the Z values when moving the camera around.
            // Bevy has a specific camera setup and this can mess with how our layers are shown.
            player_transform.translation.z = z;

            // For now just follow eagerly
            camera_transform.translation = player_transform.translation;
        }
    }

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

fn zoom_handler(mouse_input: &mut EventReader<'_, '_, MouseWheel>, ortho: &mut Mut<'_, OrthographicProjection>) {
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
