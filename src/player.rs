use bevy::{prelude::*, input::mouse::MouseWheel};
use std::cmp::{max, min};

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

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    _marker: Player,
    health: Health,
    sprite: SpriteBundle,
    transform: Transform
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
    mut player: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    // Camera
    for (mut transform, mut ortho) in camera.iter_mut() {
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

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }

}


