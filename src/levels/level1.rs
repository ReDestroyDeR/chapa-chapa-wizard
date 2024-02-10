use crate::levels::LevelConfig;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct Level1Asset {
    #[asset(path = "levels/level1.tmx")]
    pub map: Handle<crate::helpers::tiled::TiledMap>,
    #[asset(path = "levels/level1.ccwl.json")]
    pub config: Handle<LevelConfig>,
}
