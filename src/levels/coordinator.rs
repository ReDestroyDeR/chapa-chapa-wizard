use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::TilePos,
};

use crate::{helpers::coordinate_utils::CoordinateOps, player::Player};

use super::{Level, LevelConfig};

pub struct LevelCoordniatorPlugin;

impl Plugin for LevelCoordniatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(JsonAssetPlugin::<LevelConfig>::new(&["ccwl.json"]))
            .add_state::<LevelLoadingStates>()
            .add_loading_state(
                LoadingState::new(LevelLoadingStates::Loading)
                    .continue_to_state(LevelLoadingStates::Ready),
            )
            .add_systems(Update, handle_out_of_bounds);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum LevelLoadingStates {
    #[default]
    Loading,
    Ready,
}

fn handle_out_of_bounds<'a>(
    level: Query<&Level>,
    tilemap: Query<(&TilemapGridSize, &TilemapType, &TilemapSize, &Transform), Without<Player>>,
    mut moving_entities: Query<&mut Transform, (With<Player>, Changed<Transform>)>,
    level_config_assets: Res<Assets<LevelConfig>>,
) {
    level.for_each(|l| {
        tilemap.for_each(|(grid_size, map_type, map_size, map_transform)| {
            let cfg = level_config_assets
                .get(&l.cfg)
                .expect("LevelConfig not found or unexpectedly unloaded!");
            moving_entities.for_each_mut(|mut entity_transform| {
                let entity_world_pos = entity_transform.translation.xy();
                let coordinate_zero = map_transform.translation.xy().tiled_top_left(map_size, grid_size);
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
                    debug!("Not walkable {:?} {:?}", pos.x, pos.y);
                    if let Some((x, y)) = cfg
                        .walkable_tiles
                        .nearest_walkable_tiles_local((pos.x as usize, pos.y as usize))
                        .into_iter()
                        .map(|(x, y)| {
                            let tile_pos = TilePos::new(x as u32, y as u32);
                            let (x, y) = tile_pos
                                .center_in_world(grid_size, map_type)
                                .copy_signs(&Vec2::new(1., -1.))
                                .undo_relative(&coordinate_zero)
                                .into();
                            (x, y)
                        })
                        .min_by(|&a, &b| {
                            let a_dist = Vec2::from(a).distance_squared(entity_world_pos);
                            let b_dist = Vec2::from(b).distance_squared(entity_world_pos);
                            a_dist.total_cmp(&b_dist)
                        })
                    {
                        debug!("Moving to {:?} {:?}", x, y);
                        entity_transform.translation =
                            Vec3::new(x, y, entity_transform.translation.z);
                    } else {
                        warn!(
                            "Couldn't find nearest tile from pos: {:#?} to displace from {:#?}",
                            pos, entity_transform.translation
                        );
                    }
                }
            })
        })
    })
}
