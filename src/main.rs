mod helpers;
mod motd;
mod player;

use crate::motd::MotdPlugin;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use player::{PlayerPlugin, PlayerBundle, AnimationIndices, AnimationTimer};

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
        .add_plugins(helpers::tiled::TiledMapPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {
    commands.spawn(Camera2dBundle::default()); 

    let map_handle: Handle<helpers::tiled::TiledMap> = asset_server.load("levels/level1.tmx");

    commands.spawn(helpers::tiled::TiledMapBundle {
        tiled_map: map_handle,
        ..default()
    });

    let archer = asset_server.load("sprites/Factions/Knights/Troops/Archer/Blue/Archer_Blue.png");
    let atlas = 
        TextureAtlas::from_grid(archer, Vec2::new(192., 192.), 8, 7, None,  None);
    let texture_atlas_handle = texture_atlases.add(atlas);

    let animation_indices = AnimationIndices::Bounded { first: 0, last: 5 };
    commands.spawn(PlayerBundle {
        sprite: SpriteSheetBundle  {
            texture_atlas: texture_atlas_handle,
            sprite: match animation_indices {
                AnimationIndices::Bounded{first, last: _} => TextureAtlasSprite::new(first),
                _ => TextureAtlasSprite::new(0)
            },
            transform: Transform::from_xyz(0., 0., 100.),
            ..default()
        },
        animation_indices,
        animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ..default()
    });
}
