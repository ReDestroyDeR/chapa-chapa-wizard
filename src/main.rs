mod animation;
mod entities;
mod helpers;
mod levels;
mod motd;
mod player;

use crate::motd::MotdPlugin;
use animation::{AnimationBundle, AnimationLoadingStates, SpriteAnimationPlugin};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use entities::archer::ArcherBlue;
use levels::{
    coordinator::{LevelCoordniatorPlugin, LevelLoadingStates},
    level1::Level1Asset,
    Level, LevelBundle,
};
use player::{PlayerBundle, PlayerPlugin};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Chapa Chapa Wizard"),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(MotdPlugin)
        .add_plugins(TilemapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(LevelCoordniatorPlugin)
        .add_collection_to_loading_state::<_, Level1Asset>(LevelLoadingStates::Loading)
        .add_plugins(SpriteAnimationPlugin)
        .add_collection_to_loading_state::<_, ArcherBlue>(AnimationLoadingStates::LoadingSprites)
        .add_plugins(helpers::tiled::TiledMapPlugin)
        .add_systems(OnEnter(AnimationLoadingStates::Ready), animations)
        .add_systems(OnEnter(LevelLoadingStates::Ready), level)
        .run();
}

fn animations(
    mut commands: Commands,
    archer_blue_res: Res<ArcherBlue>,
    animation_bundle_assets: Res<Assets<AnimationBundle>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let atlas = TextureAtlas::from_grid(
        archer_blue_res.image.clone(),
        Vec2::new(192., 192.),
        8,
        7,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(atlas);

    commands.spawn(PlayerBundle {
        sprite: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(64. * -5., 64. * 2., 100.),
            ..default()
        },
        animations: animation_bundle_assets
            .get(&archer_blue_res.animations)
            .unwrap()
            .clone(),
        ..default()
    });
}

fn level(mut commands: Commands, level1: Res<Level1Asset>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LevelBundle {
        tilemap: helpers::tiled::TiledMapBundle {
            tiled_map: level1.map.clone(),
            ..default()
        },
        level: Level {
            cfg: level1.config.clone(),
        },
    });
}
