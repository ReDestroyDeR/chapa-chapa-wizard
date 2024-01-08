use bevy::{prelude::Handle, ecs::system::Resource, render::texture::Image};
use bevy_asset_loader::prelude::*;
use bevy::asset::AssetServer;

use crate::animation::AnimationBundle;


#[derive(AssetCollection, Resource)]
pub struct ArcherBlue {
    #[asset(path = "sprites/Factions/Knights/Troops/Archer/Blue/Archer_Blue.png")]
    pub image: Handle<Image>,
    #[asset(path = "sprites/Factions/Knights/Troops/Archer/Blue/Archer_Blue.animations.json")]
    pub animations: Handle<AnimationBundle>
}
