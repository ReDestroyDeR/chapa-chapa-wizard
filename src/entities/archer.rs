use bevy::asset::AssetServer;
use bevy::sprite::TextureAtlas;
use bevy::{ecs::system::Resource, prelude::Handle, asset::Assets};
use bevy_asset_loader::prelude::*;
use bevy::prelude::*;

use crate::animation::AnimationBundle;

#[derive(AssetCollection, Resource)]
pub struct ArcherBlue {
    #[asset(texture_atlas(tile_size_x = 192., tile_size_y = 192., columns = 8, rows = 7, padding_x = 0., padding_y = 0., offset_x = 0., offset_y = 0.))]
    #[asset(path = "sprites/Factions/Knights/Troops/Archer/Blue/Archer_Blue.png")]
    pub texture_atlas: Handle<TextureAtlas>,
    #[asset(path = "sprites/Factions/Knights/Troops/Archer/Blue/Archer_Blue.animations.json")]
    pub animations: Handle<AnimationBundle>,
}

#[derive(AssetCollection, Resource)]
pub struct ArcherRed {
    #[asset(texture_atlas(tile_size_x = 192., tile_size_y = 192., columns = 8, rows = 7, padding_x = 0., padding_y = 0., offset_x = 0., offset_y = 0.))]
    #[asset(path = "sprites/Factions/Knights/Troops/Archer/Red/Archer_Red.png")]
    pub texture_atlas: Handle<TextureAtlas>,
    #[asset(path = "sprites/Factions/Knights/Troops/Archer/Red/Archer_Red.animations.json")]
    pub animations: Handle<AnimationBundle>,
}
