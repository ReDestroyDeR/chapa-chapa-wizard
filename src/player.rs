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

#[derive(Default, Component)]
pub enum AnimationIndices {
    #[default]
    NoAnimation,
    Bounded{
        first: usize,
        last: usize,
    }
}

#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    pub _marker: Player,
    pub health: Health,
    pub sprite: SpriteSheetBundle,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {

    fn build(&self, app: &mut App) {
        app.add_systems(Update, (movement, animate_sprite));
    }

}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        if let AnimationIndices::Bounded{first, last} = indices {
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = if sprite.index == *last {
                    *first
                } else {
                    sprite.index + 1
                };
            }
        }
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
    for (mut camera_transform, mut ortho) in camera.iter_mut() {
        for mut player_transform in player.iter_mut() {
            zoom_handler(&mut mouse_input, &mut ortho);

            let z = player_transform.translation.z;
            player_transform.translation += time.delta_seconds() * direction_from(&keyboard_input) * 500.;
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


