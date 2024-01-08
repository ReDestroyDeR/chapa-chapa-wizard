mod helpers;
mod motd;
mod player;
mod animation;
mod entities;

use crate::motd::MotdPlugin;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_asset_loader::prelude::*;
use entities::archer::ArcherBlue;
use player::{PlayerPlugin, PlayerBundle};
use animation::{AnimationLoadingStates, SpriteAnimationPlugin, AnimationBundle};

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
        .add_plugins(SpriteAnimationPlugin)
        .add_collection_to_loading_state::<_, ArcherBlue>(AnimationLoadingStates::LoadingSprites)
        .add_plugins(helpers::tiled::TiledMapPlugin)
        .add_systems(OnEnter(AnimationLoadingStates::Ready), startup)
        .run();
}

fn startup(mut commands: Commands, 
           archer_blue_res: Res<ArcherBlue>,
           animation_bundle_assets: Res<Assets<AnimationBundle>>,
           asset_server: Res<AssetServer>, 
           mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {
    commands.spawn(Camera2dBundle::default()); 

    let map_handle: Handle<helpers::tiled::TiledMap> = asset_server.load("levels/level1.tmx");

    commands.spawn(helpers::tiled::TiledMapBundle {
        tiled_map: map_handle,
        ..default()
    });

    let atlas = 
        TextureAtlas::from_grid(archer_blue_res.image.clone(), Vec2::new(192., 192.), 8, 7, None,  None);
    let texture_atlas_handle = texture_atlases.add(atlas);

    commands.spawn(PlayerBundle {
        sprite: SpriteSheetBundle  {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0., 0., 100.),
            ..default()
        },
        animations: animation_bundle_assets.get(&archer_blue_res.animations).unwrap().clone(),
        ..default()
    });
}
